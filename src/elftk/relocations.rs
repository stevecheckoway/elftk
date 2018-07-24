use types::Elf_Word;

// i386
relocations!(i386_relocation_name, {
    R_386_NONE          = 0,  // none   none
    R_386_32            = 1,  // word32 S + A
    R_386_PC32          = 2,  // word32 S + A - P
    R_386_GOT32         = 3,  // word32 G + A - GOT
    R_386_PLT32         = 4,  // word32 L + A - P
    R_386_COPY          = 5,  // none   none
    R_386_GLOB_DAT      = 6,  // word32 S
    R_386_JUMP_SLOT     = 7,  // word32 S
    R_386_RELATIVE      = 8,  // word32 B + A
    R_386_GOTOFF        = 9,  // word32 S + A - GOT
    R_386_GOTPC         = 10, // word32 GOT + A - P
    R_386_TLS_TPOFF     = 14, // word32
    R_386_TLS_IE        = 15, // word32
    R_386_TLS_GOTIE     = 16, // word32
    R_386_TLS_LE        = 17, // word32
    R_386_TLS_GD        = 18, // word32
    R_386_TLS_LDM       = 19, // word32
    R_386_16            = 20, // word16 S + A
    R_386_PC16          = 21, // word16 S + A - P
    R_386_8             = 22, // word8  S + A
    R_386_PC8           = 23, // word8  S + A - P
    R_386_TLS_GD_32     = 24, // word32
    R_386_TLS_GD_PUSH   = 25, // word32
    R_386_TLS_GD_CALL   = 26, // word32
    R_386_TLS_GD_POP    = 27, // word32
    R_386_TLS_LDM_32    = 28, // word32
    R_386_TLS_LDM_PUSH  = 29, // word32
    R_386_TLS_LDM_CALL  = 30, // word32
    R_386_TLS_LDM_POP   = 31, // word32
    R_386_TLS_LDO_32    = 32, // word32
    R_386_TLS_IE_32     = 33, // word32
    R_386_TLS_LE_32     = 34, // word32
    R_386_TLS_DTPMOD32  = 35, // word32
    R_386_TLS_DTPOFF32  = 36, // word32
    R_386_TLS_TPOFF32   = 37, // word32
    R_386_SIZE32        = 38, // word32 Z + A
    R_386_TLS_GOTDESC   = 39, // word32
    R_386_TLS_DESC_CALL = 40, // none   none
    R_386_TLS_DESC      = 41, // word32
    R_386_IRELATIVE     = 42  // word32 indirect (B + A)
});

// x86-64
relocations!(x86_64_relocation_name, {
    R_X86_64_NONE            = 0,  // none none
    R_X86_64_64              = 1,  // word64 S + A
    R_X86_64_PC32            = 2,  // word32 S + A - P
    R_X86_64_GOT32           = 3,  // word32 G + A
    R_X86_64_PLT32           = 4,  // word32 L + A - P
    R_X86_64_COPY            = 5,  // none none
    R_X86_64_GLOB_DAT        = 6,  // word64 S
    R_X86_64_JUMP_SLOT       = 7,  // word64 S
    R_X86_64_RELATIVE        = 8,  // word64 B + A
    R_X86_64_GOTPCREL        = 9,  // word32 G + GOT + A - P
    R_X86_64_32              = 10, // word32 S + A
    R_X86_64_32S             = 11, // word32 S + A
    R_X86_64_16              = 12, // word16 S + A
    R_X86_64_PC16            = 13, // word16 S + A - P
    R_X86_64_8               = 14, // word8 S + A
    R_X86_64_PC8             = 15, // word8 S + A - P
    R_X86_64_DTPMOD64        = 16, // word64
    R_X86_64_DTPOFF64        = 17, // word64
    R_X86_64_TPOFF64         = 18, // word64
    R_X86_64_TLSGD           = 19, // word32
    R_X86_64_TLSLD           = 20, // word32
    R_X86_64_DTPOFF32        = 21, // word32
    R_X86_64_GOTTPOFF        = 22, // word32
    R_X86_64_TPOFF32         = 23, // word32
    R_X86_64_PC64            = 24, // word64 S + A - P
    R_X86_64_GOTOFF64        = 25, // word64 S + A - GOT
    R_X86_64_GOTPC32         = 26, // word32 GOT + A - P
    R_X86_64_SIZE32          = 32, // word32 Z + A
    R_X86_64_SIZE64          = 33, // word64 Z + A
    R_X86_64_GOTPC32_TLSDESC = 34, // word32
    R_X86_64_TLSDESC_CALL    = 35, // none
    R_X86_64_TLSDESC         = 36, // word64×2
    R_X86_64_IRELATIVE       = 37  // word64 indirect (B + A)
});
