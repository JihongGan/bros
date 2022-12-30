use core::arch::asm;

enum Syscall {
    Write = 64,
    Exit = 93,
}

fn syscall(id: Syscall, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id as usize,
        );
    }
    ret
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    syscall(Syscall::Write, [fd, buf.as_ptr() as usize, buf.len()])
}

pub fn exit(exit_code: i32) -> isize {
    syscall(Syscall::Exit, [exit_code as usize, 0, 0])
}
