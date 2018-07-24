#![allow(non_camel_case_types)]

// Common
/// 16-bit unsigned integer.
pub type Elf_Half = u16;
/// 32-bit unsigned integer.
pub type Elf_Word = u32;
/// 32-bit signed integer.
pub type Elf_Sword = i32;
/// 64-bit unsigned integer.
pub type Elf_Xword = u64;
/// 64-bit signed integer.
pub type Elf_Sxword = i64;

// 32-bit
/// 16-bit unsigned integer for 32-bit ELF object files.
pub type Elf32_Half = Elf_Half;
/// 32-bit unsigned integer for 32-bit ELF object files.
pub type Elf32_Word = Elf_Word;
/// 32-bit signed integer for 32-bit ELF object files.
pub type Elf32_Sword = Elf_Sword;
/// 32-bit unsigned offset for 32-bit ELF object files.
pub type Elf32_Off = u32;
/// 32-bit address for 32-bit ELF object files.
pub type Elf32_Addr = u32;

// 64-bit
/// 16-bit unsigned integer for 64-bit ELF object files.
pub type Elf64_Half = Elf_Half;
/// 32-bit unsigned integer for 64-bit ELF object files.
pub type Elf64_Word = Elf_Word;
/// 32-bit signed integer for 64-bit ELF object files.
pub type Elf64_Sword = Elf_Sword;
/// 64-bit unsigned integer for 64-bit ELF object files.
pub type Elf64_Xword = Elf_Xword;
/// 64-bit signed integer for 64-bit ELF object files.
pub type Elf64_Sxword = Elf_Sxword;
/// 64-bit unsigned offset for 64-bit ELF object files.
pub type Elf64_Off = u64;
/// 64-bit address for 64-bit ELF object files.
pub type Elf64_Addr = u64;

/// The size of [Elf32_Ehdr's e_ident](struct.Elf32_Ehdr.html#structfield.e_ident) and [Elf64_Ehdr's
/// e_ident](struct.Elf64_Ehdr.html#structfield.e_ident) field.
pub const EI_NIDENT: usize = 16;

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

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf32_Chdr {
    pub ch_type:      Elf32_Word,
    pub ch_size:      Elf32_Word,
    pub ch_addralign: Elf32_Word,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf64_Chdr {
    pub ch_type:      Elf64_Word,
    pub ch_reserved:  Elf64_Word,
    pub ch_size:      Elf64_Xword,
    pub ch_addralign: Elf64_Xword,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf32_Sym {
    pub st_name:  Elf32_Word,
    pub st_value: Elf32_Addr,
    pub st_size:  Elf32_Word,
    pub st_info:  u8,
    pub st_other: u8,
    pub st_shndx: Elf32_Half,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf64_Sym {
    pub st_name:  Elf64_Word,
    pub st_info:  u8,
    pub st_other: u8,
    pub st_shndx: Elf64_Half,
    pub st_value: Elf64_Addr,
    pub st_size:  Elf64_Xword,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf32_Rel {
    pub r_offset: Elf32_Addr,
    pub r_info:   Elf32_Word,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf64_Rel {
    pub r_offset: Elf64_Addr,
    pub r_info:   Elf64_Xword,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf32_Rela {
    pub r_offset: Elf32_Addr,
    pub r_info:   Elf32_Word,
    pub r_addend: Elf32_Sword,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf64_Rela {
    pub r_offset:   Elf64_Addr,
    pub r_info:     Elf64_Xword,
    pub r_addend:   Elf64_Sxword,
}

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

#[doc(hidden)]
pub unsafe trait ElfType {}
unsafe impl ElfType for Elf_Half {}
unsafe impl ElfType for Elf_Word {}
unsafe impl ElfType for Elf_Sword {}
unsafe impl ElfType for Elf_Xword {}
unsafe impl ElfType for Elf_Sxword {}
unsafe impl ElfType for Elf32_Ehdr {}
unsafe impl ElfType for Elf64_Ehdr {}
unsafe impl ElfType for Elf32_Shdr {}
unsafe impl ElfType for Elf64_Shdr {}
unsafe impl ElfType for Elf32_Chdr {}
unsafe impl ElfType for Elf64_Chdr {}
unsafe impl ElfType for Elf32_Sym {}
unsafe impl ElfType for Elf64_Sym {}
unsafe impl ElfType for Elf32_Rel {}
unsafe impl ElfType for Elf64_Rel {}
unsafe impl ElfType for Elf32_Rela {}
unsafe impl ElfType for Elf64_Rela {}
unsafe impl ElfType for Elf32_Phdr {}
unsafe impl ElfType for Elf64_Phdr {}
