#![allow(non_camel_case_types)]
#![allow(dead_code)]

// Common
pub type Elf_Half = u16;
pub type Elf_Word = u32;
pub type Elf_Sword = i32;
pub type Elf_Xword = u64;
pub type Elf_Sxword = i64;

// 32-bit
pub type Elf32_Half = Elf_Half;
pub type Elf32_Word = Elf_Word;
pub type Elf32_Sword = Elf_Sword;
pub type Elf32_Off = u32;
pub type Elf32_Addr = u32;

// 64-bit
pub type Elf64_Half = Elf_Half;
pub type Elf64_Word = Elf_Word;
pub type Elf64_Sword = Elf_Sword;
pub type Elf64_Xword = Elf_Xword;
pub type Elf64_Sxword = Elf_Sxword;
pub type Elf64_Off = u64;
pub type Elf64_Addr = u64;

pub const EI_NIDENT: usize = 16; // Size of e_ident[]

pub unsafe trait ElfStruct {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
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
unsafe impl ElfStruct for Elf32_Ehdr {}

#[derive(Clone, Copy, Debug)]
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
unsafe impl ElfStruct for Elf64_Ehdr {}


#[derive(Clone, Copy, Debug)]
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
unsafe impl ElfStruct for Elf32_Shdr {}

#[derive(Clone, Copy, Debug)]
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
unsafe impl ElfStruct for Elf64_Shdr {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf32_Chdr {
    pub ch_type:      Elf32_Word,
    pub ch_size:      Elf32_Word,
    pub ch_addralign: Elf32_Word,
}
unsafe impl ElfStruct for Elf32_Chdr {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf64_Chdr {
    pub ch_type:      Elf64_Word,
    pub ch_reserved:  Elf64_Word,
    pub ch_size:      Elf64_Xword,
    pub ch_addralign: Elf64_Xword,
}
unsafe impl ElfStruct for Elf64_Chdr {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf32_Sym {
    st_name:   Elf32_Word,
    st_value:  Elf32_Addr,
    st_size:   Elf32_Word,
    st_info:   u8,
    st_other:  u8,
    st_shndx:  Elf32_Half,
}
unsafe impl ElfStruct for Elf32_Sym {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf64_Sym {
    st_name:   Elf64_Word,
    st_info:   u8,
    st_other:  u8,
    st_shndx:  Elf64_Half,
    st_value:  Elf64_Addr,
    st_size:   Elf64_Xword,
}
unsafe impl ElfStruct for Elf64_Sym {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf32_Rel {
    r_offset:   Elf32_Addr,
    r_info:     Elf32_Word,
}
unsafe impl ElfStruct for Elf32_Rel {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf64_Rel {
    r_offset:   Elf64_Addr,
    r_info:     Elf64_Xword,
}
unsafe impl ElfStruct for Elf64_Rel {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf32_Rela {
    r_offset:   Elf32_Addr,
    r_info:     Elf32_Word,
    r_addend:   Elf32_Sword,
}
unsafe impl ElfStruct for Elf32_Rela {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf64_Rela {
    r_offset:   Elf64_Addr,
    r_info:     Elf64_Xword,
    r_addend:   Elf64_Sxword,
}
unsafe impl ElfStruct for Elf64_Rela {}

#[derive(Clone, Copy, Debug)]
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
unsafe impl ElfStruct for Elf32_Phdr {}

#[derive(Clone, Copy, Debug)]
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
unsafe impl ElfStruct for Elf64_Phdr {}
