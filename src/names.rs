use elftk as elf;

pub fn section_flags(flags: elf::Elf_Xword) -> ([u8; 15], usize) {
    let mut s = [b' '; 15];
    let mut idx = 0;
    let mut checked = 0;
    {
        let mut check_flag = |f, c| {
            if flags & f & !checked != 0 {
                s[idx] = c;
                idx += 1;
            }
            checked |= f;
        };
        check_flag(elf::SHF_WRITE,            b'W');
        check_flag(elf::SHF_ALLOC,            b'A');
        check_flag(elf::SHF_EXECINSTR,        b'X');
        check_flag(elf::SHF_MERGE,            b'M');
        check_flag(elf::SHF_STRINGS,          b'S');
        check_flag(elf::SHF_INFO_LINK,        b'I');
        check_flag(elf::SHF_LINK_ORDER,       b'L');
        check_flag(elf::SHF_OS_NONCONFORMING, b'O');
        check_flag(elf::SHF_GROUP,            b'G');
        check_flag(elf::SHF_TLS,              b'T');
        check_flag(elf::SHF_COMPRESSED,       b'C');
        check_flag(elf::SHF_MASKOS,           b'o');
        check_flag(elf::SHF_EXCLUDED,         b'E');
        check_flag(elf::SHF_MASKPROC,         b'p');
    }
    if flags & !checked != 0 {
        s[idx] = b'x';
        idx += 1;
    }
    (s, idx)
}

pub fn symbol_type(typ: u8) -> &'static str {
    match typ {
        elf::STT_NOTYPE                     => "NOTYPE",
        elf::STT_OBJECT                     => "OBJECT",
        elf::STT_FUNC                       => "FUNC",
        elf::STT_SECTION                    => "SECTION",
        elf::STT_FILE                       => "FILE",
        elf::STT_COMMON                     => "COMMON",
        elf::STT_TLS                        => "TLS",
        elf::STT_LOOS ... elf::STT_HIOS     => "OS",
        elf::STT_LOPROC ... elf::STT_HIPROC => "PROC",
        _                                   => "UNKNOWN",
    }
}

pub fn symbol_binding(binding: u8) -> &'static str {
    match binding {
        elf::STB_LOCAL                      => "LOCAL",
        elf::STB_GLOBAL                     => "GLOBAL",
        elf::STB_WEAK                       => "WEAK",
        elf::STB_LOOS ... elf::STB_HIOS     => "OS",
        elf::STB_LOPROC ... elf::STB_HIPROC => "PROC",
        _                                   => "UNKNOWN",
    }
}

pub fn symbol_visibility(vis: u8) -> &'static str {
    match vis {
        elf::STV_DEFAULT   => "DEFAULT",
        elf::STV_INTERNAL  => "INTERNAL",
        elf::STV_HIDDEN    => "HIDDEN",
        elf::STV_PROTECTED => "PROTECTED",
        _                  => unreachable!(),
    }
}

pub fn symbol_index(index: elf::SectionIndex) -> String {
    match index {
        elf::SectionIndex::Normal(ndx)               => format!("{}", ndx),
        elf::SectionIndex::Reserved(elf::SHN_ABS)    => "ABS".into(),
        elf::SectionIndex::Reserved(elf::SHN_COMMON) => "COM".into(),
        elf::SectionIndex::Reserved(elf::SHN_UNDEF)  => "UND".into(),
        elf::SectionIndex::Reserved(ndx)             => format!("?{}?", ndx),
    }
}

pub fn relocation_name(machine: elf::Elf_Half, relocation_type: elf::Elf_Word) -> &'static str {
    macro_rules! rnames {
        { $($x:ident),+ } => {
            match relocation_type {
                $(
                    elf::$x => stringify!($x)
                ),+,
                _ => "<unknown>",
            }
        }
    }
    match machine {
        elf::EM_386 => rnames! {
            R_386_NONE,
            R_386_32,
            R_386_PC32,
            R_386_GOT32,
            R_386_PLT32,
            R_386_COPY,
            R_386_GLOB_DAT,
            R_386_JUMP_SLOT,
            R_386_RELATIVE,
            R_386_GOTOFF,
            R_386_GOTPC,
            R_386_TLS_TPOFF,
            R_386_TLS_IE,
            R_386_TLS_GOTIE,
            R_386_TLS_LE,
            R_386_TLS_GD,
            R_386_TLS_LDM,
            R_386_16,
            R_386_PC16,
            R_386_8,
            R_386_PC8,
            R_386_TLS_GD_32,
            R_386_TLS_GD_PUSH,
            R_386_TLS_GD_CALL,
            R_386_TLS_GD_POP,
            R_386_TLS_LDM_32,
            R_386_TLS_LDM_PUSH,
            R_386_TLS_LDM_CALL,
            R_386_TLS_LDM_POP,
            R_386_TLS_LDO_32,
            R_386_TLS_IE_32,
            R_386_TLS_LE_32,
            R_386_TLS_DTPMOD32,
            R_386_TLS_DTPOFF32,
            R_386_TLS_TPOFF32,
            R_386_SIZE32,
            R_386_TLS_GOTDESC,
            R_386_TLS_DESC_CALL,
            R_386_TLS_DESC,
            R_386_IRELATIVE
        },
        elf::EM_X86_64 => rnames! {
            R_X86_64_NONE,
            R_X86_64_64,
            R_X86_64_PC32,
            R_X86_64_GOT32,
            R_X86_64_PLT32,
            R_X86_64_COPY,
            R_X86_64_GLOB_DAT,
            R_X86_64_JUMP_SLOT,
            R_X86_64_RELATIVE,
            R_X86_64_GOTPCREL,
            R_X86_64_32,
            R_X86_64_32S,
            R_X86_64_16,
            R_X86_64_PC16,
            R_X86_64_8,
            R_X86_64_PC8,
            R_X86_64_DTPMOD64,
            R_X86_64_DTPOFF64,
            R_X86_64_TPOFF64,
            R_X86_64_TLSGD,
            R_X86_64_TLSLD,
            R_X86_64_DTPOFF32,
            R_X86_64_GOTTPOFF,
            R_X86_64_TPOFF32,
            R_X86_64_PC64,
            R_X86_64_GOTOFF64,
            R_X86_64_GOTPC32,
            R_X86_64_SIZE32,
            R_X86_64_SIZE64,
            R_X86_64_GOTPC32_TLSDESC,
            R_X86_64_TLSDESC_CALL,
            R_X86_64_TLSDESC,
            R_X86_64_IRELATIVE
        },
        _ => "<unimplemented>"
    }
}
