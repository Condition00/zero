use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::Stream, stream::StreamExt, task::AtomicWaker};
use ps2_mouse::Mouse;
use x86_64::instructions::port::Port;

static PACKET_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

// Called by interrupt handler
pub(crate) fn add_packet(packet: u8) {
    if let Ok(queue) = PACKET_QUEUE.try_get() {
        if queue.push(packet).is_err() {
            // Queue full, drop packet
        } else {
            WAKER.wake();
        }
    }
}

/// Wait for the PS/2 controller to be ready to accept a command
unsafe fn wait_for_write() {
    let mut port = Port::new(0x64);
    for _ in 0..100000 {
        let status: u8 = port.read();
        if (status & 0x02) == 0 {
            return;
        }
    }
}

/// Wait for data to be available from the PS/2 controller
unsafe fn wait_for_read() {
    let mut port = Port::new(0x64);
    for _ in 0..100000 {
        let status: u8 = port.read();
        if (status & 0x01) != 0 {
            return;
        }
    }
}

/// Write a command to the mouse
unsafe fn write_mouse(command: u8) {
    let mut cmd_port = Port::new(0x64);
    let mut data_port = Port::new(0x60);

    wait_for_write();
    cmd_port.write(0xD4u8); // Tell controller next byte goes to mouse
    wait_for_write();
    data_port.write(command);
}

/// Read data from the mouse
unsafe fn read_mouse_data() -> u8 {
    wait_for_read();
    let mut data_port = Port::new(0x60);
    data_port.read()
}

// Initialize the mouse
pub fn init() {
    PACKET_QUEUE
        .try_init_once(|| ArrayQueue::new(100))
        .expect("mouse queue already initialized");

    // Enable mouse on PS/2 controller
    unsafe {
        let mut cmd_port = Port::new(0x64);
        let mut data_port = Port::new(0x60);

        // Enable auxiliary device (mouse)
        wait_for_write();
        cmd_port.write(0xA8u8);

        // Get compaq status byte
        wait_for_write();
        cmd_port.write(0x20u8);
        wait_for_read();
        let mut status: u8 = data_port.read();

        // Set compaq status byte (enable IRQ12)
        status |= 0x02; // Enable mouse interrupts
        status &= !0x20; // Enable mouse clock

        wait_for_write();
        cmd_port.write(0x60u8);
        wait_for_write();
        data_port.write(status);

        // Send command to mouse to set defaults
        write_mouse(0xF6);
        read_mouse_data(); // Read ACK

        // Enable mouse data reporting
        write_mouse(0xF4);
        read_mouse_data(); // Read ACK
    }
}

pub struct PacketStream {
    _private: (),
}

impl PacketStream {
    pub fn new() -> Self {
        PacketStream { _private: () }
    }
}

impl Stream for PacketStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = PACKET_QUEUE
            .try_get()
            .expect("packet queue not initialized");

        // Fast path
        if let Some(packet) = queue.pop() {
            return Poll::Ready(Some(packet));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(packet) => {
                WAKER.take();
                Poll::Ready(Some(packet))
            }
            None => Poll::Pending,
        }
    }
}

// Async function to process mouse events
pub async fn handle_mouse_events() {
    let mut packets = PacketStream::new();
    let mut mouse = Mouse::new();

    while let Some(packet) = packets.next().await {
        // process_packet modifies internal state
        mouse.process_packet(packet);

        // Get the current state (always returns a state)
        let state = mouse.get_state();
        handle_mouse_state(state);
    }
}

fn handle_mouse_state(state: ps2_mouse::MouseState) {
    use crate::println;

    // Check button states
    if state.left_button_down() {
        println!("🖱️  Left button pressed!");
    }

    if state.right_button_down() {
        println!("🖱️  Right button pressed!");
    }

    // Check movement
    let x = state.get_x();
    let y = state.get_y();

    if x != 0 || y != 0 {
        println!("🖱️  Mouse moved: X={:+4}, Y={:+4}", x, y);
    }
}
