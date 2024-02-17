mod elf;

fn main() {
    let header = elf::Elf64_Ehdr {
        e_ident: [
            elf::ELFMAG0,
            elf::ELFMAG1,
            elf::ELFMAG2,
            elf::ELFMAG3,
            elf::ELFCLASS64,
            elf::ELFDATA2LSB,
            elf::EV_CURRENT,
            elf::ELFOSABI_SYSV,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        e_type: elf::ET_EXEC,
        e_machine: elf::EM_X86_64,
        e_entry: 0x40007f,
        e_phoff: 64,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 64,
        e_phentsize: 56,
        e_phnum: 1,
        e_shentsize: 64,
        e_shnum: 0,
        e_shstrndx: elf::SHN_UNDEF,
        e_version: 0,
    };
    let phdr = elf::Elf64_Phdr {
        p_type: elf::PT_LOAD,
        p_offset: 0x78, // 64 + 56 = 120
        p_vaddr: 0x400078,
        p_paddr: 0x400078,
        p_filesz: 44,
        p_memsz: 44,
        p_flags: elf::PF_X | elf::PF_R,
        p_align: 0x8,
    };

    let code: [u8; 44] = [
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21, 0xa,  // Hello!\n
        0xb8, // mov rax (32bit)
        1, 0, 0, 0,    // write syscall 1
        0xbf, // mov rdi (32 bit)
        1, 0, 0, 0, // stdout
        0x48, 0xbe, // mov rsi, 64 bit pointer
        0x78, 0, 0x40, 0, 0, 0, 0, 0,    // Hello strings address in virtual memory
        0xba, // mov rdx (32bit)
        7, 0, 0, 0, // number of bytes in Hello!\n
        0xf, 0x5,  // syscall
        0xb8, // mov rax (32 bit)
        0x3c, 0, 0, 0, // 60 = exit syscall
        0x48, 0x31, 0xff, // xor rdi, rdi
        0xf, 0x5, // syscall
    ];
    let mut bytes = Vec::<u8>::new();
    bytes.extend(header.to_bytes());
    bytes.extend(phdr.to_bytes());
    bytes.extend(&code);
    std::fs::write("./out", &bytes).unwrap();
}
