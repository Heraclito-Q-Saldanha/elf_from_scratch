#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl Elf64_Phdr {
    #[inline]
    pub const fn to_bytes(self) -> [u8; 56] {
        let p_type = self.p_type.to_ne_bytes();
        let p_flags = self.p_flags.to_ne_bytes();
        let p_offset = self.p_offset.to_ne_bytes();
        let p_vaddr = self.p_vaddr.to_ne_bytes();
        let p_paddr = self.p_paddr.to_ne_bytes();
        let p_filesz = self.p_filesz.to_ne_bytes();
        let p_memsz = self.p_memsz.to_ne_bytes();
        let p_align = self.p_align.to_ne_bytes();
        [
            p_type[0],
            p_type[1],
            p_type[2],
            p_type[3],
            p_flags[0],
            p_flags[1],
            p_flags[2],
            p_flags[3],
            p_offset[0],
            p_offset[1],
            p_offset[2],
            p_offset[3],
            p_offset[4],
            p_offset[5],
            p_offset[6],
            p_offset[7],
            p_vaddr[0],
            p_vaddr[1],
            p_vaddr[2],
            p_vaddr[3],
            p_vaddr[4],
            p_vaddr[5],
            p_vaddr[6],
            p_vaddr[7],
            p_paddr[0],
            p_paddr[1],
            p_paddr[2],
            p_paddr[3],
            p_paddr[4],
            p_paddr[5],
            p_paddr[6],
            p_paddr[7],
            p_filesz[0],
            p_filesz[1],
            p_filesz[2],
            p_filesz[3],
            p_filesz[4],
            p_filesz[5],
            p_filesz[6],
            p_filesz[7],
            p_memsz[0],
            p_memsz[1],
            p_memsz[2],
            p_memsz[3],
            p_memsz[4],
            p_memsz[5],
            p_memsz[6],
            p_memsz[7],
            p_align[0],
            p_align[1],
            p_align[2],
            p_align[3],
            p_align[4],
            p_align[5],
            p_align[6],
            p_align[7],
        ]
    }
}

impl Elf64_Ehdr {
    #[inline]
    pub const fn to_bytes(self) -> [u8; 64] {
        let e_machine = self.e_machine.to_ne_bytes();
        let e_type = self.e_type.to_ne_bytes();
        let e_version = self.e_version.to_ne_bytes();
        let e_entry = self.e_entry.to_ne_bytes();
        let e_phoff = self.e_phoff.to_ne_bytes();
        let e_shoff = self.e_shoff.to_ne_bytes();
        let e_flags = self.e_flags.to_ne_bytes();
        let e_ehsize = self.e_ehsize.to_ne_bytes();
        let e_phentsize = self.e_phentsize.to_ne_bytes();
        let e_phnum = self.e_phnum.to_ne_bytes();
        let e_shentsize = self.e_shentsize.to_ne_bytes();
        let e_shnum = self.e_shnum.to_ne_bytes();
        let e_shstrndx = self.e_shstrndx.to_ne_bytes();
        [
            self.e_ident[0],
            self.e_ident[1],
            self.e_ident[2],
            self.e_ident[3],
            self.e_ident[4],
            self.e_ident[5],
            self.e_ident[6],
            self.e_ident[7],
            self.e_ident[8],
            self.e_ident[9],
            self.e_ident[10],
            self.e_ident[11],
            self.e_ident[12],
            self.e_ident[13],
            self.e_ident[14],
            self.e_ident[15],
            e_type[0],
            e_type[1],
            e_machine[0],
            e_machine[1],
            e_version[0],
            e_version[1],
            e_version[2],
            e_version[3],
            e_entry[0],
            e_entry[1],
            e_entry[2],
            e_entry[3],
            e_entry[4],
            e_entry[5],
            e_entry[6],
            e_entry[7],
            e_phoff[0],
            e_phoff[1],
            e_phoff[2],
            e_phoff[3],
            e_phoff[4],
            e_phoff[5],
            e_phoff[6],
            e_phoff[7],
            e_shoff[0],
            e_shoff[1],
            e_shoff[2],
            e_shoff[3],
            e_shoff[4],
            e_shoff[5],
            e_shoff[6],
            e_shoff[7],
            e_flags[0],
            e_flags[1],
            e_flags[2],
            e_flags[3],
            e_ehsize[0],
            e_ehsize[1],
            e_phentsize[0],
            e_phentsize[1],
            e_phnum[0],
            e_phnum[1],
            e_shentsize[0],
            e_shentsize[1],
            e_shnum[0],
            e_shnum[1],
            e_shstrndx[0],
            e_shstrndx[1],
        ]
    }
}

pub const EI_NIDENT: usize = 16;

pub const ELFMAG0: u8 = 0x7f;

pub const ELFMAG1: u8 = 0x45;

pub const ELFMAG2: u8 = 0x4c;
pub const EM_X86_64: u16 = 62;
pub const ET_EXEC: u16 = 2;

pub const ELFMAG3: u8 = 0x46;

pub const ELFCLASS64: u8 = 2;

pub const ELFDATA2LSB: u8 = 1;

pub const ELFOSABI_SYSV: u8 = 0;

pub const EV_CURRENT: u8 = 1;

pub const PF_X: u32 = 1;

pub const PF_R: u32 = 4;

pub const PT_LOAD: u32 = 1;

pub const SHN_UNDEF: u16 = 0;
