#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::mem;
use std::iter;

use constants::*;
use error::*;
use format::*;
use types::*;

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
                ElfT::Elf32LE(s) => $t32::from_le(s.$field).into(),
                ElfT::Elf32BE(s) => $t32::from_be(s.$field).into(),
                ElfT::Elf64LE(s) => $t64::from_le(s.$field),
                ElfT::Elf64BE(s) => $t64::from_be(s.$field),
            }
        }
    }
}

pub type ElfHeaderRef<'a> = ElfRef<'a, Elf32_Ehdr, Elf64_Ehdr>;

impl<'a> ElfHeaderRef<'a> {
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

#[derive(Debug, Copy, Clone)]
pub struct Sections<'a, 'b, I> where
    'a: 'b,
    I: Iterator<Item=SectionHeaderRef<'a>>,
{
    reader: &'b Reader<'a>,
    header_iter: I,
}

impl<'a, 'b, I> Iterator for Sections<'a, 'b, I> where
    'a: 'b,
    I: Iterator<Item=SectionHeaderRef<'a>>,
{
    type Item = SectionRef<'a>;

    fn next(&mut self) -> Option<SectionRef<'a>> {
        if let Some(shdr) = self.header_iter.next() {
            self.reader.get_section(shdr).ok()
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.header_iter.size_hint()
    }
}

#[derive(Debug, Clone)]
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
                value: data[EI_VERSION].into(),
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
                    value: x.into(),
                });
            },
            (x, ELFDATA2LSB) | (x, ELFDATA2MSB) => {
                return Err(ElfError::InvalidHeaderField {
                    header: "ELF",
                    field: "e_ident[EI_CLASS]",
                    value: x.into(),
                });
            },
            _ => unreachable!(),
        };
        let ehdr = ElfHeaderRef::try_from(ehdr_data)?;

        if ehdr.e_version() != EV_CURRENT {
            return Err(ElfError::InvalidHeaderField {
                header: "ELF",
                field: "e_version",
                value: ehdr.e_version().into(),
            });
        }

        let mut reader = Reader {
            data,
            ehdr,
            symtab_index: SHN_UNDEF.into(),
            dynsym_index: SHN_UNDEF.into(),
            dynamic_index: SHN_UNDEF.into(),
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
            let size = u64::from(reader.ehdr.e_phentsize());
            let num = u64::from(reader.ehdr.e_phnum());
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
            let size = u64::from(reader.ehdr.e_shentsize());
            let shnum = reader.ehdr.e_shnum();
            let shstrndx = reader.ehdr.e_shstrndx();

            if shnum >= SHN_LORESERVE {
                return Err(ElfError::InvalidHeaderField {
                    header:"ELF",
                    field: "e_shnum",
                    value: shnum.into(),
                });
            }
            if shstrndx >= SHN_LORESERVE && shstrndx != SHN_XINDEX {
                return Err(ElfError::InvalidHeaderField {
                    header:"ELF",
                    field: "e_shstrndx",
                    value: shstrndx.into(),
                });
            }

            // If e_shnum == 0 (resp. e_shstrndx == SHN_XINDEX), then there must be at least one
            // section header in the table whose st_size (resp. st_link) member holds the real
            // number of sections (resp. index of the section string table).
            if (shnum == 0 || shstrndx == SHN_XINDEX) && !array_in_bounds(shoff, size, 1) {
                return Err(ElfError::NotContainedInFile { what: "section headers", which: shoff });
            }
            let num = reader.num_sections();
            if !array_in_bounds(shoff, size, num.into()) {
                return Err(ElfError::NotContainedInFile { what: "section headers", which: shoff+size*u64::from(num) });
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
                        value: shdr.sh_link().into(),
                    });
                }
                if shdr.sh_flags() & SHF_INFO_LINK != 0 && shdr.sh_info() >= num {
                    return Err(ElfError::InvalidHeaderField {
                        header: "section",
                        field: "sh_info",
                        value: shdr.sh_info().into(),
                    });
                }

                let index = index as Elf_Word;
                match section_type {
                    SHT_SYMTAB => {
                        if reader.symtab_index != Elf_Word::from(SHN_UNDEF) {
                            return Err(ElfError::MultipleSections { section: "SYMTAB" });
                        }
                        reader.symtab_index = index;
                    },
                    SHT_DYNSYM => {
                        if reader.dynsym_index != Elf_Word::from(SHN_UNDEF) {
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
        self.ehdr
    }

    pub fn program_headers(&self) -> ProgramHeadersRef<'a> {
        let phoff = self.ehdr.e_phoff() as usize;
        let phentsize = self.ehdr.e_phentsize() as usize;
        let phnum = if phoff == 0 { 0 } else { self.ehdr.e_phnum() as usize };
        let len = phentsize * phnum;
        let phdr_data = &self.data[phoff..phoff+len];
        ProgramHeadersRef::try_from(self.ehdr.construct_from(phdr_data)).unwrap()
    }

    pub fn segment_data(&self, phdr: ProgramHeaderRef<'a>) -> Option<&'a [u8]> {
        if phdr.p_type() == PT_NULL {
            return None;
        }
        let offset = phdr.p_offset() as usize;
        let size = phdr.p_filesz() as usize;
        Some(&self.data[offset..offset+size])
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
            shnum.into()
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
            Some(shstrndx.into())
        }
    }

    fn section_string_table(&self) -> ElfResult<Option<StringTableRef<'a>>> {
        self.section_string_table_index()
            .map_or(Ok(None), |shstrndx| {
                let shstr_shdr = self.section_headers().get(shstrndx as usize)?;
                if shstr_shdr.sh_type() != SHT_STRTAB {
                    return Err(ElfError::InvalidSectionType { expected: SHT_STRTAB, actual: shstr_shdr.sh_type() });
                }
                match self.section_data(shstr_shdr)? {
                    SectionDataRef::StringTable(strtab) => Ok(Some(strtab)),
                    _                                   => unreachable!(),
                }
            })
    }
    pub fn section_headers(&self) -> SectionHeadersRef<'a> {
        let shoff = self.ehdr.e_shoff() as usize;
        let shentsize = self.ehdr.e_shentsize() as usize;
        let shnum = self.num_sections() as usize;
        let len = shentsize * shnum;
        let shdr_data = &self.data[shoff..shoff+len];
        SectionHeadersRef::try_from(self.ehdr.construct_from(shdr_data)).unwrap()
    }

    pub fn sections<'b>(&'b self) -> Sections<'a, 'b, impl Iterator<Item=SectionHeaderRef<'a>>> {
        self.get_sections(self.section_headers().into_iter())
    }

    //pub fn sections_matching<'b, Predicate>(&'b self, pred: Predicate) -> Sections<'a, 'b, impl Iterator<Item=SectionHeaderRef<'a>>> where
    //    'a: 'b,
    //    Predicate: Fn(&SectionHeaderRef<'b>) -> bool + 'b,
    //{
    //    self.get_sections(self.section_headers().into_iter().filter(pred))
    //}

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

    pub fn uninterpreted_section_data(&self, shdr: SectionHeaderRef<'a>) -> SectionDataRef<'a> {
        match shdr.sh_type() {
            SHT_NULL | SHT_NOBITS => SectionDataRef::NoBits,
            _ => {
                let offset = shdr.sh_offset() as usize;
                let size = shdr.sh_size() as usize;
                SectionDataRef::Uninterpreted(&self.data[offset..offset+size])
            }
        }
    }

    fn linked_string_table(&self, index: Elf_Word) -> ElfResult<StringTableRef<'a>> {
        let data = self.section_headers().get(index as usize)
            .and_then(|shdr| self.section_data(shdr))
            .map_err(|_| ElfError::InvalidLinkedSection { linked: index })?;
        if let SectionDataRef::StringTable(strtab) = data {
            Ok(strtab)
        } else {
            Err(ElfError::InvalidLinkedSection { linked: index })
        }
    }

    fn linked_symbol_table(&self, index: Elf_Word) -> ElfResult<SymbolTableRef<'a>> {
        let data = self.section_headers().get(index as usize)
            .and_then(|shdr| self.section_data(shdr))
            .map_err(|_| ElfError::InvalidLinkedSection { linked: index })?;
        if let SectionDataRef::SymbolTable(symtab) = data {
            Ok(symtab)
        } else {
            Err(ElfError::InvalidLinkedSection { linked: index })
        }
    }

    fn section_data(&self, shdr: SectionHeaderRef<'a>) -> ElfResult<SectionDataRef<'a>> {
        let data = if let SectionDataRef::Uninterpreted(data) = self.uninterpreted_section_data(shdr) {
            data
        } else {
            return Ok(SectionDataRef::NoBits);
        };
        Ok(match shdr.sh_type() {
            SHT_NULL | SHT_NOBITS   => unreachable!(),
            SHT_STRTAB              => SectionDataRef::StringTable(StringTableRef { data }),
            SHT_SYMTAB | SHT_DYNSYM => {
                let symbol_names = self.linked_string_table(shdr.sh_link())?;
                let entries = SymbolTableEntriesRef::try_from(shdr.construct_from(data))?;
                let shndx = None; // TODO: 
                SectionDataRef::SymbolTable(SymbolTableRef {
                    symbol_names,
                    entries,
                    shndx,
                })
            },
            SHT_REL => SectionDataRef::RelocationTable(RelTableRef {
                symbol_table: self.linked_symbol_table(shdr.sh_link())?,
                entries: RelTableEntriesRef::try_from(shdr.construct_from(data))?,
            }),
            SHT_RELA => SectionDataRef::ExplicitRelocationTable(RelaTableRef {
                symbol_table: self.linked_symbol_table(shdr.sh_link())?,
                entries: RelaTableEntriesRef::try_from(shdr.construct_from(data))?,
            }),
            SHT_NOTE => SectionDataRef::NoteTable(NoteTableRef {
                words: ElfSliceRef::try_from(shdr.construct_from(data))?,
                data
            }),
            // TODO: Fill in the rest.
            _ => SectionDataRef::Uninterpreted(data),
        })
    }

    /// Returns a reference to the section corresponding to the section header.
    pub fn get_section(&self, shdr: SectionHeaderRef<'a>) -> ElfResult<SectionRef<'a>> {
        let name = self.section_string_table()?
            .and_then(|strtab| strtab.get_string(shdr.sh_name()));
        let data = self.section_data(shdr)?;
        Ok(SectionRef { name, shdr, data })
    }

    /// Returns a [Sections][#Sections] iterator that iterates over the sections corresponding to
    /// the section headers returned by `iter`.
    pub fn get_sections<'b, I>(&'b self, iter: I) -> Sections<'a, 'b, I> where
        'a: 'b,
        I: Iterator<Item=SectionHeaderRef<'a>>,
    {
        Sections {
            reader: self,
            header_iter: iter,
        }
    }

    pub fn symtab(&self) -> ElfResult<Option<SectionRef<'a>>> {
        if self.symtab_index == Elf_Word::from(SHN_UNDEF) {
            return Ok(None);
        }
        let shdr = self.section_headers().get(self.symtab_index as usize)?;
        Ok(Some(self.get_section(shdr)?))
    }

    pub fn dynsym(&self) -> ElfResult<Option<SectionRef<'a>>> {
        if self.dynsym_index == Elf_Word::from(SHN_UNDEF) {
            return Ok(None);
        }
        let shdr = self.section_headers().get(self.dynsym_index as usize)?;
        Ok(Some(self.get_section(shdr)?))
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

#[derive(Debug, Clone, Copy)]
pub enum SectionIndex {
    Normal(Elf_Word),
    Reserved(Elf_Half),
}

pub struct SymbolRef<'a> {
    pub symbol_name: Option<&'a [u8]>,
    pub section: SectionIndex,
    pub value: Elf64_Addr,
    pub size: Elf64_Xword,
    pub info: u8,
    pub other: u8,
}

impl<'a> SymbolRef<'a> {
    pub fn binding(&self) -> u8 {
        (self.info >> 4) & 0xf
    }

    pub fn symbol_type(&self) -> u8 {
        (self.info & 0xf)
    }

    pub fn visibility(&self) -> u8 {
        self.other & 3
    }
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
    symbol_names: StringTableRef<'a>,
    entries: SymbolTableEntriesRef<'a>,
    shndx: Option<ElfSliceRef<'a, Elf_Word, Elf_Word>>,
}

impl<'a> SymbolTableRef<'a> {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get<'b>(&'b self, index: usize) -> ElfResult<SymbolRef<'a>> where
        'a: 'b
    {
        let entry = self.entries.get(index)?;
        let shndx = entry.st_shndx();
        let section;
        if shndx < SHN_LORESERVE {
            section = SectionIndex::Normal(shndx.into());
        } else if shndx == SHN_XINDEX {
            if let Some(shndx) = self.shndx {
                section = SectionIndex::Normal(shndx.get(index)?.value());
            } else {
                return Err(ElfError::Msg { msg: "No associated SYMTAB_SHNDX section for symbol" });
            }
        } else {
            section = SectionIndex::Reserved(shndx);
        }
        let symbol_name = self.symbol_names.get_string(entry.st_name());
        Ok(SymbolRef {
            symbol_name,
            section,
            value: entry.st_value(),
            size: entry.st_size(),
            info: entry.st_info(),
            other: entry.st_other(),
        })
    }
}

pub type RelTableEntryRef<'a> = ElfRef<'a, Elf32_Rel, Elf64_Rel>;
pub type RelTableEntriesRef<'a> = ElfSliceRef<'a, Elf32_Rel, Elf64_Rel>;
pub type RelaTableEntryRef<'a> = ElfRef<'a, Elf32_Rela, Elf64_Rela>;
pub type RelaTableEntriesRef<'a> = ElfSliceRef<'a, Elf32_Rela, Elf64_Rela>;

impl<'a> RelTableEntryRef<'a> {
    field_impl!(r_offset, Elf32_Addr, Elf64_Addr);
    field_impl!(r_info,   Elf32_Word, Elf64_Xword);

    pub fn symbol_index(&self) -> Elf_Word {
        let info = self.r_info();
        self.apply(|_| info >> 8, |_| info >> 32) as Elf_Word
    }

    pub fn relocation_type(&self) -> Elf_Word {
        let info = self.r_info();
        self.apply(|_| info & 0xff, |_| info & 0xffff_ffff) as Elf_Word
    }
}

impl<'a> RelaTableEntryRef<'a> {
    field_impl!(r_offset, Elf32_Addr,  Elf64_Addr);
    field_impl!(r_info,   Elf32_Word,  Elf64_Xword);
    field_impl!(r_addend, Elf32_Sword, Elf64_Sxword);

    pub fn symbol_index(&self) -> Elf_Word {
        let info = self.r_info();
        self.apply(|_| info >> 8, |_| info >> 32) as Elf_Word
    }

    pub fn relocation_type(&self) -> Elf_Word {
        let info = self.r_info();
        self.apply(|_| info & 0xff, |_| info & 0xffff_ffff) as Elf_Word
    }
}

pub struct RelTableRef<'a> {
    pub symbol_table: SymbolTableRef<'a>,
    pub entries: RelTableEntriesRef<'a>
}

pub struct RelaTableRef<'a> {
    pub symbol_table: SymbolTableRef<'a>,
    pub entries: RelaTableEntriesRef<'a>,
}

pub struct NoteRef<'a> {
    pub name: Option<&'a [u8]>,
    pub desc: Option<&'a [u8]>,
    pub note_type: Elf_Xword,
}

pub type MachineWordRef<'a> = ElfRef<'a, Elf32_Word, Elf64_Xword>;
pub type MachineWordsRef<'a> = ElfSliceRef<'a, Elf32_Word, Elf64_Xword>;

pub struct NoteIter<'a> {
    word_iter: ElfIter<'a, Elf32_Word, Elf64_Xword>,
    data: &'a [u8],
}

impl<'a> Iterator for NoteIter<'a> {
    type Item = NoteRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let namesz = if let Some(namesz) = self.word_iter.next().map(|w| w.get()) {
            namesz as usize
        } else {
            return None;
        };
        let descsz = if let Some(descsz) = self.word_iter.next().map(|w| w.get()) {
            descsz as usize
        } else {
            return None;
        };
        let note_type = if let Some(nt) = self.word_iter.next().map(|w| w.get()) {
            nt
        } else {
            return None;
        };
        let (size, mask) = if self.word_iter.is_64bit() { (8, 7) } else { (4, 3) };
        let mut offset = 3*size;
        let total_size = offset + ((namesz + mask) & !mask) + ((descsz + mask) & !mask);
        if self.data.len() < total_size {
            return None;
        }
        // If namesz or descsz is 0, then the corresponding note doesn't have the field.
        // Otherwise, they're the length of a null-terminated string of bytes.
        let name;
        if namesz == 0 {
            name = None;
        } else {
            name = Some(&self.data[offset .. offset+namesz-1]);
            offset += (namesz + mask) & !mask;
        }
        let desc;
        if descsz == 0 {
            desc = None;
        } else {
            desc = Some(&self.data[offset .. offset+descsz-1]);
            offset += (descsz + mask) & !mask;
        }
        self.data = &self.data[offset ..];
        Some(NoteRef { name, desc, note_type })
    }
}

pub struct NoteTableRef<'a> {
    words: ElfSliceRef<'a, Elf32_Word, Elf64_Xword>,
    data: &'a [u8],
}

impl<'a> NoteTableRef<'a> {
    pub fn iter(&self) -> NoteIter<'a> {
        NoteIter {
            data: self.data,
            word_iter: self.words.iter(),
        }
    }
}

impl<'a> iter::IntoIterator for NoteTableRef<'a> {
    type Item = NoteRef<'a>;
    type IntoIter = NoteIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        NoteIter {
            data: self.data,
            word_iter: self.words.into_iter(),
        }
    }
}

pub enum SectionDataRef<'a> {
    /// Section holds a string table.
    StringTable(StringTableRef<'a>),

    /// Section holds a symbol table.
    SymbolTable(SymbolTableRef<'a>),

    /// Section holds a relocation table with implicit addends (Rel).
    RelocationTable(RelTableRef<'a>),

    /// Section holds a relocation table with explicit addends (Rela).
    ExplicitRelocationTable(RelaTableRef<'a>),

    /// Section holds a slice of Elf_Words.
    ElfWords(ElfWordsRef<'a>),

    /// Section holds a notes table.
    NoteTable(NoteTableRef<'a>),

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
section_type!(SymbolHashTableSectionRef, SHT_HASH);
section_type!(PointerArraySectionRef, SHT_INIT_ARRAY | SHT_FINI_ARRAY | SHT_PREINIT_ARRAY);
section_type!(SymbolTableSectionIndexSectionRef, SHT_SYMTAB_SHNDX);
*/
