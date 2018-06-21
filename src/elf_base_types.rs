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
