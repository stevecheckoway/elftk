#![allow(non_camel_case_types)]
#![allow(dead_code)]

use elf_base_types::*;

pub const EI_NIDENT: usize = 16; /// Size of e_ident[]

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Ehdr {
    pub e_ident:        [u8; EI_NIDENT],
    pub e_type:         Elf32_Half,
    pub e_machine:      Elf32_Half,
    pub e_version:      Elf32_Word,
    pub e_entry:        Elf32_Addr,
    pub e_phoff:        Elf32_Off,
    pub e_shoff:        Elf32_Off,
    pub e_flags:        Elf32_Word,
    pub e_ehsize:       Elf32_Half,
    pub e_phentsize:    Elf32_Half,
    pub e_phnum:        Elf32_Half,
    pub e_shentsize:    Elf32_Half,
    pub e_shnum:        Elf32_Half,
    pub e_shstrndx:     Elf32_Half,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident:        [u8; EI_NIDENT],
    pub e_type:         Elf64_Half,
    pub e_machine:      Elf64_Half,
    pub e_version:      Elf64_Word,
    pub e_entry:        Elf64_Addr,
    pub e_phoff:        Elf64_Off,
    pub e_shoff:        Elf64_Off,
    pub e_flags:        Elf64_Word,
    pub e_ehsize:       Elf64_Half,
    pub e_phentsize:    Elf64_Half,
    pub e_phnum:        Elf64_Half,
    pub e_shentsize:    Elf64_Half,
    pub e_shnum:        Elf64_Half,
    pub e_shstrndx:     Elf64_Half,
}


#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Shdr {
    pub sh_name:        Elf32_Word,
    pub sh_type:        Elf32_Word,
    pub sh_flags:       Elf32_Word,
    pub sh_addr:        Elf32_Addr,
    pub sh_offset:      Elf32_Off,
    pub sh_size:        Elf32_Word,
    pub sh_link:        Elf32_Word,
    pub sh_info:        Elf32_Word,
    pub sh_addralign:   Elf32_Word,
    pub sh_entsize:     Elf32_Word,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Shdr {
    pub sh_name:        Elf64_Word,
    pub sh_type:        Elf64_Word,
    pub sh_flags:       Elf64_Xword,
    pub sh_addr:        Elf64_Addr,
    pub sh_offset:      Elf64_Off,
    pub sh_size:        Elf64_Xword,
    pub sh_link:        Elf64_Word,
    pub sh_info:        Elf64_Word,
    pub sh_addralign:   Elf64_Xword,
    pub sh_entsize:     Elf64_Xword,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Chdr {
    pub ch_type:      Elf32_Word,
    pub ch_size:      Elf32_Word,
    pub ch_addralign: Elf32_Word,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Chdr {
    pub ch_type:      Elf64_Word,
    pub ch_reserved:  Elf64_Word,
    pub ch_size:      Elf64_Xword,
    pub ch_addralign: Elf64_Xword,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Sym {
    st_name:   Elf32_Word,
    st_value:  Elf32_Addr,
    st_size:   Elf32_Word,
    st_info:   u8,
    st_other:  u8,
    st_shndx:  Elf32_Half,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Sym {
    st_name:   Elf64_Word,
    st_info:   u8,
    st_other:  u8,
    st_shndx:  Elf64_Half,
    st_value:  Elf64_Addr,
    st_size:   Elf64_Xword,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Rel {
    r_offset:   Elf32_Addr,
    r_info:     Elf32_Word,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Rel {
    r_offset:   Elf64_Addr,
    r_info:     Elf64_Xword,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Rela {
    r_offset:   Elf32_Addr,
    r_info:     Elf32_Word,
    r_addend:   Elf32_Sword,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Rela {
    r_offset:   Elf64_Addr,
    r_info:     Elf64_Xword,
    r_addend:   Elf64_Sxword,
}


#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Phdr {
    pub p_type:   Elf32_Word,
    pub p_offset: Elf32_Off,
    pub p_vaddr:  Elf32_Addr,
    pub p_paddr:  Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz:  Elf32_Word,
    pub p_flags:  Elf32_Word,
    pub p_align:  Elf32_Word,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type:   Elf64_Word,
    pub p_flags:  Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr:  Elf64_Addr,
    pub p_paddr:  Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz:  Elf64_Xword,
    pub p_align:  Elf64_Xword,
}
