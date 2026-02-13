# Change Notes

# Previous :
swooooooosh I forgot but I somehow wrote the code. basic kernel with a ramfs and simple shell works. I refactored the code directories on 24 Jan

# Jan 25 :

working on the user memory space and intgrating the syscalls. I faced a major wrong approach when I saw resources for remapping the existing kernel memory but it caused a page fault no matter what 
later figured out that separate memory address was to be allocated to the userspace. finally was able to implement ring0-ring3 transition using iretq stack frame ahhhh

# Jan 26 :

completed the implementation of the syscalls from ring0 to ring3. Added the syscall handler with the basic functions (plannig to add more now) !important I forgot to add System Call Extentions. added some assembly to match the C calling convention. basically the foundation for user programs is running. 

# Jan 27 :
working on os sysdeps with a forked mlibc submodule. integrated mlibc for project but complete build was hard so installed only headers and now I have to write a minimal libc using these headers to run userprograms

# Feb 14 :
learnt bit of assembly and will see how to write custom wrapper. removed the incomplete mlibc library

added ps2 mouse support
