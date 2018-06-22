use super::{Elf32_Word, Elf64_Xword};

// i386
const R_386_NONE:     Elf32_Word = 0;  // none    none
const R_386_32:       Elf32_Word = 1;  // word32  S + A
const R_386_PC:       Elf32_Word = 3;  // word32  S + A - P
const R_386_GOT:      Elf32_Word = 3;  // word32  G + A - P
const R_386_PLT:      Elf32_Word = 3;  // word32  L + A - P
const R_386_COPY:     Elf32_Word = 5;  // none    none
const R_386_GLOB_DAT: Elf32_Word = 6;  // word32  S
const R_386_JMP_SLOT: Elf32_Word = 7;  // word32  S
const R_386_RELATIVE: Elf32_Word = 8;  // word32  B + A
const R_386_GOTOFF:   Elf32_Word = 9;  // word32  S + A - GOT
const R_386_GOTPC:    Elf32_Word = 10; // word32  GOT + A - P

// x86-64
const R_X86_64_NONE:             Elf64_Xword = 0;  // none none
const R_X86_64_64:               Elf64_Xword = 1;  // word64 S + A
const R_X86_64_PC32:             Elf64_Xword = 2;  // word32 S + A - P
const R_X86_64_GOT32:            Elf64_Xword = 3;  // word32 G + A
const R_X86_64_PLT32:            Elf64_Xword = 4;  // word32 L + A - P
const R_X86_64_COPY:             Elf64_Xword = 5;  // none none
const R_X86_64_GLOB_DAT:         Elf64_Xword = 6;  // word64 S
const R_X86_64_JUMP_SLOT:        Elf64_Xword = 7;  // word64 S
const R_X86_64_RELATIVE:         Elf64_Xword = 8;  // word64 B + A
const R_X86_64_GOTPCREL:         Elf64_Xword = 9;  // word32 G + GOT + A - P
const R_X86_64_32:               Elf64_Xword = 10; // word32 S + A
const R_X86_64_32S:              Elf64_Xword = 11; // word32 S + A
const R_X86_64_16:               Elf64_Xword = 12; // word16 S + A
const R_X86_64_PC16:             Elf64_Xword = 13; // word16 S + A - P
const R_X86_64_8:                Elf64_Xword = 14; // word8 S + A
const R_X86_64_PC8:              Elf64_Xword = 15; // word8 S + A - P
const R_X86_64_DTPMOD64:         Elf64_Xword = 16; // word64
const R_X86_64_DTPOFF64:         Elf64_Xword = 17; // word64
const R_X86_64_TPOFF64:          Elf64_Xword = 18; // word64
const R_X86_64_TLSGD:            Elf64_Xword = 19; // word32
const R_X86_64_TLSLD:            Elf64_Xword = 20; // word32
const R_X86_64_DTPOFF32:         Elf64_Xword = 21; // word32
const R_X86_64_GOTTPOFF:         Elf64_Xword = 22; // word32
const R_X86_64_TPOFF32:          Elf64_Xword = 23; // word32
const R_X86_64_PC64:             Elf64_Xword = 24; // word64 S + A - P
const R_X86_64_GOTOFF64:         Elf64_Xword = 25; // word64 S + A - GOT
const R_X86_64_GOTPC32:          Elf64_Xword = 26; // word32 GOT + A - P
const R_X86_64_SIZE32:           Elf64_Xword = 32; // word32 Z + A
const R_X86_64_SIZE64:           Elf64_Xword = 33; // word64 Z + A
const R_X86_64_GOTPC32_TLSDESC:  Elf64_Xword = 34; // word32
const R_X86_64_TLSDESC_CALL:     Elf64_Xword = 35; // none
const R_X86_64_TLSDESC:          Elf64_Xword = 36; // word64×2
const R_X86_64_IRELATIVE:        Elf64_Xword = 37; // word64 indirect (B + A)
