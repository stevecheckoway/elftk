#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::mem;

use super::constants::*;
use super::error::*;
use super::format::*;
use super::types::*;

macro_rules! noswap_field_impl {
    ( $field:ident, $type:ty ) => {
        pub fn $field(&self) -> $type {
            match *self {
                ElfT::Elf32LE(s) => s.$field,
                ElfT::Elf32BE(s) => s.$field,
                ElfT::Elf64LE(s) => s.$field,
                ElfT::Elf64BE(s) => s.$field,
            }
        }
    }
}
macro_rules! field_impl {
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
    //pub fn e_ident(&self) -> &[u8; EI_NIDENT] {
    //    self.apply(|hdr| &hdr.e_ident, |hdr| &hdr.e_ident)
    //}
    noswap_field_impl!(e_ident, [u8; EI_NIDENT]);
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

    pub fn segment_data(&self, phdr: ProgramHeaderRef<'a>) -> &'a [u8] {
        if phdr.p_type() == PT_NULL {
            return &[][..];
        }
        let offset = phdr.p_offset() as usize;
        let size = phdr.p_filesz() as usize;
        &self.data[offset..offset+size]
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

    fn section_string_table(&self) -> ElfResult<Option<StringTableRef<'a>>> {
        if let Some(shstrndx) = self.section_string_table_index() {
            let shstr_shdr = self.section_headers().get(shstrndx as usize)?;
            if shstr_shdr.sh_type() != SHT_STRTAB {
                return Err(ElfError::InvalidSectionType { expected: SHT_STRTAB, actual: shstr_shdr.sh_type() });
            }
            if let SectionDataRef::StringTable(strtab) = self.get_section_data(shstr_shdr)? {
                Ok(Some(strtab))
            } else {
                unreachable!()
            }
        } else {
            Ok(None)
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

    fn get_section_data(&self, shdr: SectionHeaderRef<'a>) -> ElfResult<SectionDataRef<'a>> {
        let sh_type = shdr.sh_type();
        let bytes = match sh_type {
            SHT_NULL | SHT_NOBITS => return Ok(SectionDataRef::NoBits),
            _ => {
                let offset = shdr.sh_offset() as usize;
                let size = shdr.sh_size() as usize;
                &self.data[offset..offset+size]
            }
        };
        let section_headers = self.section_headers();
        Ok(match sh_type {
            SHT_NULL | SHT_NOBITS   => unreachable!(),
            SHT_STRTAB              => SectionDataRef::StringTable(StringTableRef { data: bytes }),
            SHT_SYMTAB | SHT_DYNSYM => {
                let section_names = self.section_string_table()?;
                let symbol_names_shdr = section_headers.get(shdr.sh_link() as usize)
                    .map_err(|_| ElfError::InvalidLinkedSection { linked: shdr.sh_link() })?;
                if symbol_names_shdr.sh_type() != SHT_STRTAB {
                    return Err(ElfError::InvalidLinkedSection { linked: shdr.sh_link() });
                }
                let symbol_names = match self.get_section_data(symbol_names_shdr)? {
                    SectionDataRef::StringTable(strtab) => strtab,
                    _                                   => unreachable!(),
                };
                let entries = SymbolTableEntriesRef::try_from(shdr.construct_from(bytes))?;
                let shndx = None; // TODO: 
                SectionDataRef::SymbolTable(SymbolTableRef {
                    section_names: section_names,
                    symbol_names: symbol_names,
                    entries: entries,
                    shndx: shndx,
                })
            },
            // TODO: Fill in the rest.
            _ => SectionDataRef::Uninterpreted(bytes),
        })
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

    // XXX: Best to not make this public/remove it entirely.
    pub fn section_name<'b>(&'b self, shdr: SectionHeaderRef<'a>) -> &'a [u8] where
        'a: 'b,
    {
        if let Ok(Some(strtab)) = self.section_string_table() {
            strtab.get_string(shdr.sh_name())
                .unwrap_or(&[][..])
        } else {
            &[][..]
        }
    }
}

pub struct StringTableRef<'a> {
    data: &'a [u8],
}

impl<'a> StringTableRef<'a> {
    pub fn get_string<'b>(&'b self, index: Elf_Word) -> Option<&'a [u8]> where
        'a: 'b,
    {
        let index = index as usize;
        self.data.iter()
            .skip(index)
            .position(|&x| x == 0)
            .map(|len| &self.data[index..index+len])
    }
}

pub type SymbolTableEntryRef<'a> = ElfRef<'a, Elf32_Sym, Elf64_Sym>;
pub type SymbolTableEntriesRef<'a> = ElfSliceRef<'a, Elf32_Sym, Elf64_Sym>;

impl<'a> SymbolTableEntryRef<'a> {
    field_impl!(st_name,  Elf32_Word, Elf64_Word);
    field_impl!(st_value, Elf32_Addr, Elf64_Addr);
    field_impl!(st_size,  Elf32_Word, Elf64_Xword);
    field_impl!(st_info,  u8,         u8);
    field_impl!(st_other, u8,         u8);
    field_impl!(st_shndx, Elf32_Half, Elf64_Half);
}

pub struct SymbolRef<'a> {
    pub symbol_name: Option<&'a [u8]>,
    pub section_name: Option<&'a [u8]>,
    pub section: Option<Elf_Word>,
    pub value: Elf64_Addr,
    pub size: Elf64_Xword,
    pub info: u8,
    pub other: u8,
}

pub type ElfWordRef<'a> = ElfRef<'a, Elf32_Word, Elf64_Word>;
pub type ElfWordsRef<'a> = ElfSliceRef<'a, Elf32_Word, Elf64_Word>;

impl<'a> ElfWordRef<'a> {
    pub fn value(&self) -> Elf_Word {
        match *self {
            ElfT::Elf32LE(s) => Elf32_Word::from_le(*s),
            ElfT::Elf32BE(s) => Elf32_Word::from_be(*s),
            ElfT::Elf64LE(s) => Elf64_Word::from_le(*s),
            ElfT::Elf64BE(s) => Elf64_Word::from_be(*s),
        }
    }
}


pub struct SymbolTableRef<'a> {
    section_names: Option<StringTableRef<'a>>,
    symbol_names: StringTableRef<'a>,
    entries: SymbolTableEntriesRef<'a>,
    shndx: Option<ElfSliceRef<'a, Elf_Word, Elf_Word>>,
}

impl<'a> SymbolTableRef<'a> {
    pub fn get<'b>(&'b self, index: usize) -> ElfResult<SymbolRef<'a>> where
        'a: 'b
    {
        let entry = self.entries.get(index)?;
        let shndx = entry.st_shndx();
        let section;
        if shndx < SHN_LORESERVE {
            section = Some(shndx as Elf_Word);
        } else if shndx == SHN_XINDEX {
            if let Some(shndx) = self.shndx {
                section = Some(shndx.get(index)?.value());
            } else {
                return Err(ElfError::Msg { msg: "No associated SYMTAB_SHNDX section for symbol" });
            }
        } else {
            section = None;
        }
        let section_name = section
            .and_then(|section| self.section_names.as_ref()
            .and_then(|names| names.get_string(section)));
        let symbol_name = self.symbol_names.get_string(entry.st_name());
        Ok(SymbolRef {
            symbol_name: symbol_name,
            section_name: section_name,
            section: section,
            value: entry.st_value(),
            size: entry.st_size(),
            info: entry.st_info(),
            other: entry.st_other(),
        })
    }
}

pub enum SectionDataRef<'a> {
    /// Section holds a string table.
    StringTable(StringTableRef<'a>),

    /// Section holds a symbol table.
    SymbolTable(SymbolTableRef<'a>),

    /// Section holds a slice of Elf_Words.
    ElfWords(ElfWordsRef<'a>),

    /// Section holds some uninterpreted data.
    Uninterpreted(&'a [u8]),

    /// Section has no data in the file.
    NoBits,
}

pub struct SectionRef<'a> {
    pub shdr: SectionHeaderRef<'a>,
    pub name: Option<&'a [u8]>,
    pub data: SectionDataRef<'a>,
}

/*
section_type!(StringTableSectionRef, SHT_STRTAB);
section_type!(SymbolTableSectionRef, SHT_SYMTAB | SHT_DYNSYM);
section_type!(RelaSectionRef, SHT_RELA);
section_type!(RelSectionRef, SHT_REL);
section_type!(NoteSectionRef, SHT_NOTE);
section_type!(SymbolHashTableSectionRef, SHT_HASH);
section_type!(PointerArraySectionRef, SHT_INIT_ARRAY | SHT_FINI_ARRAY | SHT_PREINIT_ARRAY);
section_type!(SymbolTableSectionIndexSectionRef, SHT_SYMTAB_SHNDX);
*/
