#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::mem;

use super::constants::*;
use super::error::*;
use super::format::*;
use super::types::*;

macro_rules! field_impl {
    ( $field:ident, u8, u8) => {
        pub fn $field(&self) -> u8 {
            match *self {
                ElfT::Elf32LE(s) => s.$field,
                ElfT::Elf32BE(s) => s.$field,
                ElfT::Elf64LE(s) => s.$field,
                ElfT::Elf64BE(s) => s.$field,
            }
        }
    };
    ( $field:ident, $t32:ident, $t64:ident ) => {
        pub fn $field(&self) -> $t64 {
            match *self {
                ElfT::Elf32LE(s) => $t32::from_le(s.$field) as $t64,
                ElfT::Elf32BE(s) => $t32::from_be(s.$field) as $t64,
                ElfT::Elf64LE(s) => $t64::from_le(s.$field),
                ElfT::Elf64BE(s) => $t64::from_be(s.$field),
            }
        }
    }
}

pub type ElfHeaderRef<'a> = ElfRef<'a, Elf32_Ehdr, Elf64_Ehdr>;

impl<'a> ElfHeaderRef<'a> {
    pub fn e_ident(&self) -> &[u8; EI_NIDENT] {
        self.apply(|hdr| &hdr.e_ident, |hdr| &hdr.e_ident)
    }

    field_impl!(e_type,      Elf32_Half, Elf64_Half);
    field_impl!(e_machine,   Elf32_Half, Elf64_Half);
    field_impl!(e_version,   Elf32_Word, Elf64_Word);
    field_impl!(e_entry,     Elf32_Addr, Elf64_Addr);
    field_impl!(e_phoff,     Elf32_Off,  Elf64_Off);
    field_impl!(e_shoff,     Elf32_Off,  Elf64_Off);
    field_impl!(e_flags,     Elf32_Word, Elf64_Word);
    field_impl!(e_ehsize,    Elf32_Half, Elf64_Half);
    field_impl!(e_phentsize, Elf32_Half, Elf64_Half);
    field_impl!(e_phnum,     Elf32_Half, Elf64_Half);
    field_impl!(e_shentsize, Elf32_Half, Elf64_Half);
    field_impl!(e_shnum,     Elf32_Half, Elf64_Half);
    field_impl!(e_shstrndx,  Elf32_Half, Elf64_Half);
}

pub type ProgramHeaderRef<'a> = ElfRef<'a, Elf32_Phdr, Elf64_Phdr>;
pub type ProgramHeadersRef<'a> = ElfSliceRef<'a, Elf32_Phdr, Elf64_Phdr>;

impl<'a> ProgramHeaderRef<'a> {
    field_impl!(p_type,   Elf32_Word, Elf64_Word);
    field_impl!(p_offset, Elf32_Off,  Elf64_Off);
    field_impl!(p_vaddr,  Elf32_Addr, Elf64_Addr);
    field_impl!(p_paddr,  Elf32_Addr, Elf64_Addr);
    field_impl!(p_filesz, Elf32_Word, Elf64_Xword);
    field_impl!(p_memsz,  Elf32_Word, Elf64_Xword);
    field_impl!(p_flags,  Elf32_Word, Elf64_Word);
    field_impl!(p_align,  Elf32_Word, Elf64_Xword);

    /*
    pub fn segment_data(&self) -> &[u8] {
        let offset = self.p_offset() as usize;
        let size = self.p_filesz() as usize;
        let elf_data = self.extra();
        &elf_data[offset..offset+size]
    }
    */
}

pub type SectionHeaderRef<'a> = ElfRef<'a, Elf32_Shdr, Elf64_Shdr>;
pub type SectionHeadersRef<'a> = ElfSliceRef<'a, Elf32_Shdr, Elf64_Shdr>;

impl<'a> SectionHeaderRef<'a> {
    field_impl!(sh_name,      Elf32_Word, Elf64_Word);
    field_impl!(sh_type,      Elf32_Word, Elf64_Word);
    field_impl!(sh_flags,     Elf32_Word, Elf64_Xword);
    field_impl!(sh_addr,      Elf32_Addr, Elf64_Addr);
    field_impl!(sh_offset,    Elf32_Off,  Elf64_Off);
    field_impl!(sh_size,      Elf32_Word, Elf64_Xword);
    field_impl!(sh_link,      Elf32_Word, Elf64_Word);
    field_impl!(sh_info,      Elf32_Word, Elf64_Word);
    field_impl!(sh_addralign, Elf32_Word, Elf64_Xword);
    field_impl!(sh_entsize,   Elf32_Word, Elf64_Xword);

    /*
    pub fn section_data<'b>(&'b self) -> &'a[u8] where
        'a: 'b,
    {
        let section_type = self.sh_type();
        if section_type == SHT_NULL || section_type == SHT_NOBITS {
            return &[][..];
        }
        let offset = self.sh_offset() as usize;
        let size = self.sh_size() as usize;
        let elf_data = self.extra();
        &elf_data[offset..offset+size]
    }

    #[inline]
    fn reader<'b>(&'b self) -> Reader<'a> where
        'a: 'b,
    {
        Reader { data: self.extra() }
    }

    pub fn section_name<'b>(&'b self) -> &'a [u8] where
        'a: 'b,
    {
        let reader = self.reader();
        reader.section_string_table_index()
            .and_then(|index| StringTableSectionRef::from_section(reader.sections().get(index).unwrap()))
            .and_then(|str_table| str_table.get_string(self.sh_name()))
            .unwrap_or(&[][..])
    }
    */
}

pub struct Reader<'a> {
    data: &'a [u8],
    ehdr: ElfHeaderRef<'a>,
    symtab_index: Elf_Word,
    dynsym_index: Elf_Word,
    dynamic_index: Elf_Word,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> ElfResult<Reader<'a>> {
        // Check the ELF header.
        if data.len() < mem::size_of::<Elf32_Ehdr>() ||
           &data[0..4] != b"\x7fELF"
        {
            return Err(ElfError::NotElfFile);
        }

        // ELF version
        if data[EI_VERSION] != EV_CURRENT as u8 {
            return Err(ElfError::InvalidHeaderField {
                header: "ELF",
                field: "e_ident[EI_VERSION]",
                value: data[EI_VERSION] as u64,
            });
        }

        // Try to construct the ELF header.
        let ehdr_data = match (data[EI_CLASS], data[EI_DATA]) {
            (ELFCLASS32, ELFDATA2LSB) => ElfT::Elf32LE(&data[0..mem::size_of::<Elf32_Ehdr>()]),
            (ELFCLASS32, ELFDATA2MSB) => ElfT::Elf32BE(&data[0..mem::size_of::<Elf32_Ehdr>()]),
            (ELFCLASS64, ELFDATA2LSB) => ElfT::Elf64LE(&data[0..mem::size_of::<Elf64_Ehdr>()]),
            (ELFCLASS64, ELFDATA2MSB) => ElfT::Elf64BE(&data[0..mem::size_of::<Elf64_Ehdr>()]),
            (ELFCLASS32, x) | (ELFCLASS64, x) => {
                return Err(ElfError::InvalidHeaderField {
                    header: "ELF",
                    field: "e_ident[EI_DATA]",
                    value: x as u64,
                });
            },
            (x, ELFDATA2LSB) | (x, ELFDATA2MSB) => {
                return Err(ElfError::InvalidHeaderField {
                    header: "ELF",
                    field: "e_ident[EI_CLASS]",
                    value: x as u64,
                });
            },
            _ => unreachable!(),
        };
        let ehdr = ElfHeaderRef::try_from(ehdr_data)?;

        if ehdr.e_version() != EV_CURRENT {
            return Err(ElfError::InvalidHeaderField {
                header: "ELF",
                field: "e_version",
                value: ehdr.e_version() as u64,
            });
        }

        let mut reader = Reader {
            data: data,
            ehdr: ehdr,
            symtab_index: SHN_UNDEF as Elf_Word,
            dynsym_index: SHN_UNDEF as Elf_Word,
            dynamic_index: SHN_UNDEF as Elf_Word,
        };

        // Check the program and section headers.
        let len = data.len() as u64;
        let is_64bit = reader.is_64bit();

        let array_in_bounds = move |offset: u64, size: u64, num: u64| -> bool {
            size.checked_mul(num)
                .and_then(|total_size| offset.checked_add(total_size))
                .map_or(false, move |end| end <= len)
        };

        if len < reader.ehdr.e_entry() {
            return Err(ElfError::NotContainedInFile {
                what: "ELF header field e_entry",
                which: reader.ehdr.e_entry(),
            });
        }

        // Check the program headers, if any.
        let phoff = reader.ehdr.e_phoff();
        if phoff > 0 {
            let size = reader.ehdr.e_phentsize() as u64;
            let num = reader.ehdr.e_phnum() as u64;
            if !array_in_bounds(phoff, size, num) {
                return Err(ElfError::NotContainedInFile { what: "program headers", which: phoff });
            }
            if (is_64bit && size as usize != mem::size_of::<Elf64_Phdr>()) ||
               (!is_64bit && size as usize != mem::size_of::<Elf32_Phdr>())
            {
                return Err(ElfError::InvalidHeaderField {
                    header: "ELF",
                    field: "e_phentsize",
                    value: size,
                });
            }
            // Check that the segments are contained in the file
            for (index, phdr) in reader.program_headers().into_iter().enumerate() {
                if !array_in_bounds(phdr.p_offset(), phdr.p_filesz(), 1) {
                    return Err(ElfError::NotContainedInFile { what: "segment", which: index as u64});
                }
            }

        }
        let shoff = reader.ehdr.e_shoff();
        if shoff > 0 {
            let size = reader.ehdr.e_shentsize() as u64;
            let shnum = reader.ehdr.e_shnum();
            let shstrndx = reader.ehdr.e_shstrndx();

            if shnum >= SHN_LORESERVE {
                return Err(ElfError::InvalidHeaderField {
                    header:"ELF",
                    field: "e_shnum",
                    value: shnum as u64,
                });
            }
            if shstrndx >= SHN_LORESERVE && shstrndx != SHN_XINDEX {
                return Err(ElfError::InvalidHeaderField {
                    header:"ELF",
                    field: "e_shstrndx",
                    value: shstrndx as u64,
                });
            }

            // If e_shnum == 0 (resp. e_shstrndx == SHN_XINDEX), then there must be at least one
            // section header in the table whose st_size (resp. st_link) member holds the real
            // number of sections (resp. index of the section string table).
            if (shnum == 0 || shstrndx == SHN_XINDEX) && !array_in_bounds(shoff, size, 1) {
                return Err(ElfError::NotContainedInFile { what: "section headers", which: shoff });
            }
            let num = reader.num_sections();
            if !array_in_bounds(shoff, size, num as u64) {
                return Err(ElfError::NotContainedInFile { what: "section headers", which: shoff+size*num as u64 });
            }

            if (is_64bit && size as usize != mem::size_of::<Elf64_Shdr>()) ||
               (!is_64bit && size as usize != mem::size_of::<Elf32_Shdr>())
            {
                return Err(ElfError::InvalidHeaderField {
                    header: "ELF",
                    field: "e_shentsize",
                    value: size,
                });
            }

            // Check that the sections are contained in the file.
            let sections = reader.section_headers();
            for (index, shdr) in sections.iter().enumerate() {
                let section_type = shdr.sh_type();
                if section_type != SHT_NOBITS && section_type != SHT_NULL &&
                   !array_in_bounds(shdr.sh_offset(), shdr.sh_size(), 1)
                {
                    return Err(ElfError::NotContainedInFile { what: "section", which: index as u64 });
                }
                if shdr.sh_link() >= num {
                    return Err(ElfError::InvalidHeaderField {
                        header: "section",
                        field: "sh_link",
                        value: shdr.sh_link() as u64,
                    });
                }
                if shdr.sh_flags() & SHF_INFO_LINK != 0 && shdr.sh_info() >= num {
                    return Err(ElfError::InvalidHeaderField {
                        header: "section",
                        field: "sh_info",
                        value: shdr.sh_info() as u64,
                    });
                }

                let index = index as Elf_Word;
                match section_type {
                    SHT_SYMTAB => {
                        if reader.symtab_index != SHN_UNDEF as Elf_Word {
                            return Err(ElfError::MultipleSections { section: "SYMTAB" });
                        }
                        reader.symtab_index = index;
                    },
                    SHT_DYNSYM => {
                        if reader.dynsym_index != SHN_UNDEF as Elf_Word {
                            return Err(ElfError::MultipleSections { section: "DYNSYM" });
                        }
                        reader.dynsym_index = index;
                    },
                    _ => {},
                }
            }
        }
        Ok(reader)
    }

    pub fn little_endian(&self) -> bool {
        self.data[EI_DATA] == ELFDATA2LSB
    }

    pub fn is_64bit(&self) -> bool {
        self.data[EI_CLASS] == ELFCLASS64
    }

    pub fn header(&self) -> ElfHeaderRef<'a> {
        self.ehdr.clone()
    }

    pub fn program_headers(&self) -> ProgramHeadersRef<'a> {
        let phoff = self.ehdr.e_phoff() as usize;
        let phentsize = self.ehdr.e_phentsize() as usize;
        let phnum = if phoff == 0 { 0 } else { self.ehdr.e_phnum() as usize };
        let len = phentsize * phnum;
        let phdr_data = &self.data[phoff..phoff+len];
        ProgramHeadersRef::try_from(self.ehdr.construct_from(phdr_data)).unwrap()
    }

    // The ELF file MUST have sections if this file is called.
    fn section_0(&self) -> SectionHeaderRef<'a> {
        let shoff = self.ehdr.e_shoff() as usize;
        let shentsize = self.ehdr.e_shentsize() as usize;
        debug_assert!(shoff > 0);
        let shdr_data = &self.data[shoff..shoff+shentsize];
        SectionHeaderRef::try_from(self.ehdr.construct_from(shdr_data)).unwrap()
    }

    pub fn num_sections(&self) -> Elf_Word {
        // If e_shoff > 0 and e_shnum() == 0, then section 0's st_size member holds the actual
        // number of sections.
        let shoff = self.ehdr.e_shoff();
        if shoff == 0 {
            return 0;
        }
        let shnum = self.ehdr.e_shnum();
        if shnum > 0 {
            shnum as Elf_Word
        } else {
            self.section_0().sh_size() as Elf_Word
        }
    }

    pub fn section_string_table_index(&self) -> Option<Elf_Word> {
        let shstrndx = self.ehdr.e_shstrndx();
        if shstrndx == SHN_UNDEF {
            return None;
        }
        if shstrndx == SHN_XINDEX {
            Some(self.section_0().sh_link())
        } else {
            Some(shstrndx as Elf_Word)
        }
    }

    pub fn section_headers(&self) -> SectionHeadersRef<'a> {
        let shoff = self.ehdr.e_shoff() as usize;
        let shentsize = self.ehdr.e_shentsize() as usize;
        let shnum = self.num_sections() as usize;
        let len = shentsize * shnum;
        let shdr_data = &self.data[shoff..shoff+len];
        SectionHeadersRef::try_from(self.ehdr.construct_from(shdr_data)).unwrap()
    }

    pub fn section_data<'b>(&'b self, shdr: SectionHeaderRef<'a>) -> &'a [u8] where
        'a: 'b,
    {
        match shdr.sh_type() {
            SHT_NULL | SHT_NOBITS => &[][..],
            _ => {
                let offset = shdr.sh_offset() as usize;
                let size = shdr.sh_size() as usize;
                &self.data[offset..offset+size]
            }
        }
    }

    pub fn section_name<'b>(&'b self, shdr: SectionHeaderRef<'a>) -> &'a [u8] where
        'a: 'b,
    {
        // XXX: Use a string table method.
        self.section_string_table_index()
            .and_then(|index| {
                let strtab_shdr = self.section_headers().get(index as usize).unwrap();
                if strtab_shdr.sh_type() != SHT_STRTAB {
                    return None;
                }
                let data = self.section_data(strtab_shdr);
                let index = shdr.sh_name() as usize;
                data.iter().skip(index).position(|&x| x == 0)
                    .map(|len| &data[index..index+len])
            })
            .unwrap_or(&[][..])
        //self.section_string_table_index()
        //    .and_then(|index| StringTableSectionRef::from_section(reader.sections().get(index).unwrap()))
        //    .and_then(|str_table| str_table.get_string(self.sh_name()))
        //    .unwrap_or(&[][..])
    }
}


/*
macro_rules! facade {
    { $($name:ident : $t64:ty),* } => {
        $( #[inline] pub fn $name(&self) -> $t64 { (self.0).$name() } )*
    }
}

macro_rules! section_type {
    ($name:ident, $($type:pat)|+) => {
        pub struct $name<'a>(SectionRef<'a>);
        impl<'a> $name<'a> {
            facade! {
                sh_name:      Elf64_Word,
                sh_type:      Elf64_Word,
                sh_flags:     Elf64_Xword,
                sh_addr:      Elf64_Addr,
                sh_offset:    Elf64_Off,
                sh_size:      Elf64_Xword,
                sh_link:      Elf64_Word,
                sh_info:      Elf64_Word,
                sh_addralign: Elf64_Xword,
                sh_entsize:   Elf64_Xword,
                section_data: &'a [u8],
                section_name: &'a [u8]
            }

            pub fn into_section(self) -> SectionRef<'a> { self.0 }

            pub fn from_section(sec: SectionRef<'a>) -> Option<Self> {
                match sec.sh_type() {
                    $($type)|+ => Some($name(sec)),
                    _          => None,
                }
            }
        }
    }
}

section_type!(StringTableSectionRef, SHT_STRTAB);
section_type!(SymbolTableSectionRef, SHT_SYMTAB | SHT_DYNSYM);
section_type!(RelaSectionRef, SHT_RELA);
section_type!(RelSectionRef, SHT_REL);
section_type!(NoteSectionRef, SHT_NOTE);
section_type!(SymbolHashTableSectionRef, SHT_HASH);
section_type!(PointerArraySectionRef, SHT_INIT_ARRAY | SHT_FINI_ARRAY | SHT_PREINIT_ARRAY);
section_type!(SymbolTableSectionIndexSectionRef, SHT_SYMTAB_SHNDX);

impl<'a> StringTableSectionRef<'a> {
    pub fn get_string<'b>(&'b self, index: Elf_Word) -> Option<&'a [u8]> where
        'a: 'b,
    {
        let index = index as usize;
        let data = self.section_data();
        data.iter().skip(index).position(|&x| x == 0)
            .map(|len| &data[index..index+len])
    }
}

pub type SymbolTableEntryRef<'a> = ElfRef<'a, Elf32_Sym, Elf64_Sym, &'a [u8]>;
pub type SymbolTableRef<'a> = ElfSliceRef<'a, Elf32_Sym, Elf64_Sym, &'a [u8]>;

impl<'a> SymbolTableEntryRef<'a> {
    field_impl!(st_name,  Elf32_Word, Elf64_Word);
    field_impl!(st_value, Elf32_Addr, Elf64_Addr);
    field_impl!(st_size,  Elf32_Word, Elf64_Xword);
    field_impl!(st_info,  u8,         u8);
    field_impl!(st_other, u8,         u8);
    field_impl!(st_shndx, Elf32_Half, Elf64_Half);

    pub fn name<'b>(&'b self) -> Option<&'a [u8]> {
        None
    }
}

impl<'a> SymbolTableSectionRef<'a> {
}
*/
