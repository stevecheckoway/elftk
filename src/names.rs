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
    match machine {
        elf::EM_386    => elf::i386_relocation_name(relocation_type),
        elf::EM_X86_64 => elf::x86_64_relocation_name(relocation_type),
        _              => "<unimplemented>",
    }
}
