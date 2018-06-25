#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::io;
use std::mem;
use std::slice;

use super::constants::*;
use super::format::*;
use super::types::*;

macro_rules! field_impl {
    ( $field:ident, $t32:ident, $t64:ident ) => {
        pub fn $field(&self) -> $t64 {
            match *self {
                ElfT::Elf32LE(hdr, _) => $t32::from_le(hdr.$field) as $t64,
                ElfT::Elf32BE(hdr, _) => $t32::from_be(hdr.$field) as $t64,
                ElfT::Elf64LE(hdr, _) => $t64::from_le(hdr.$field),
                ElfT::Elf64BE(hdr, _) => $t64::from_be(hdr.$field),
            }
        }
    }
}

pub type HeaderRef<'a> = ElfRef<'a, Elf32_Ehdr, Elf64_Ehdr, ()>;

impl<'a> HeaderRef<'a> {
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

pub type SegmentRef<'a> = ElfRef<'a, Elf32_Phdr, Elf64_Phdr, &'a [u8]>;
pub type SegmentsRef<'a> = ElfSliceRef<'a, Elf32_Phdr, Elf64_Phdr, &'a [u8]>;

impl<'a> SegmentRef<'a> {
    field_impl!(p_type,   Elf32_Word, Elf64_Word);
    field_impl!(p_offset, Elf32_Off,  Elf64_Off);
    field_impl!(p_vaddr,  Elf32_Addr, Elf64_Addr);
    field_impl!(p_paddr,  Elf32_Addr, Elf64_Addr);
    field_impl!(p_filesz, Elf32_Word, Elf64_Xword);
    field_impl!(p_memsz,  Elf32_Word, Elf64_Xword);
    field_impl!(p_flags,  Elf32_Word, Elf64_Word);
    field_impl!(p_align,  Elf32_Word, Elf64_Xword);

    pub fn segment_data(&self) -> &[u8] {
        let offset = self.p_offset() as usize;
        let size = self.p_filesz() as usize;
        let elf_data = self.extra();
        &elf_data[offset..offset+size]
    }
}

pub type SectionRef<'a> = ElfRef<'a, Elf32_Shdr, Elf64_Shdr, &'a [u8]>;
pub type SectionsRef<'a> = ElfSliceRef<'a, Elf32_Shdr, Elf64_Shdr, &'a [u8]>;

impl<'a> SectionRef<'a> {
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
}

pub struct Reader<'a> {
    data: &'a [u8],
}

fn invalid_data<T> (msg: &str) -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::InvalidData, msg))
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> io::Result<Reader<'a>> {
        // Check the ELF header.
        if data.len() < mem::size_of::<Elf32_Ehdr>() ||
           &data[0..4] != b"\x7fELF"
        {
            return invalid_data("Not an ELF file");
        }

        // ELF class
        if data[EI_CLASS] != ELFCLASS32 && data[EI_CLASS] != ELFCLASS64 {
            return invalid_data(&format!("Unknown ELF class {}", data[EI_CLASS]));
        }
        if data[EI_CLASS] == ELFCLASS64 && data.len() < mem::size_of::<Elf64_Ehdr>() {
            return invalid_data("Not an ELF file");
        }

        // ELF data (endianness)
        if data[EI_DATA] != ELFDATA2LSB && data[EI_DATA] != ELFDATA2MSB {
            return invalid_data(&format!("Unknown ELF data (endianness) {}", data[EI_DATA]));
        }

        // ELF version
        if data[EI_VERSION] != EV_CURRENT as u8 {
            return invalid_data(&format!("Unknown ELF header version {}", data[EI_VERSION]));
        }

        let reader = Reader { data: data };
        Reader::validate_length(&reader)?;
        Ok(reader)
    }

    fn validate_length(reader: &Reader<'a>) -> io::Result<()> {
        let hdr = reader.header();
        let len = reader.data.len() as u64;
        let is_64bit = reader.is_64bit();

        let array_in_bounds = move |offset: u64, size: u64, num: u64| -> bool {
            size.checked_mul(num)
                .and_then(|total_size| offset.checked_add(total_size))
                .map_or(false, move |end| end <= len)
        };

        if len < hdr.e_entry() {
            return invalid_data("File too small for entry point");
        }

        // Check the program headers, if any.
        let phoff = hdr.e_phoff();
        if phoff > 0 {
            let size = hdr.e_phentsize() as u64;
            let num = hdr.e_phnum() as u64;
            if !array_in_bounds(phoff, size, num) {
                return invalid_data("File too small for program headers");
            }
            if (is_64bit && size as usize != mem::size_of::<Elf64_Phdr>()) ||
               (!is_64bit && size as usize != mem::size_of::<Elf32_Phdr>())
            {
                return invalid_data(&format!("Unexpected program header size {} for {}-bit ELF",
                                             size, { if is_64bit { "64" } else { "32" } }));
            }
            // Check that the segments are contained in the file
            for (index, segment) in reader.segments().into_iter().enumerate() {
                if !array_in_bounds(segment.p_offset(), segment.p_filesz(), 1) {
                    return invalid_data(&format!("Segment {} not contained within file", index));
                }
            }

        }
        let shoff = hdr.e_shoff();
        if shoff > 0 {
            let size = hdr.e_shentsize() as u64;
            let shnum = hdr.e_shnum();
            let shstrndx = hdr.e_shstrndx();

            if shnum >= SHN_LORESERVE {
                return invalid_data(&format!("ELF header member e_shnum = {} is invalid", shnum));
            }
            if shstrndx >= SHN_LORESERVE && shstrndx != SHN_XINDEX {
                return invalid_data(&format!("ELF header member e_shstrndx = {} is invalid", shstrndx));
            }

            // If e_shnum == 0 (resp. e_shstrndx == SHN_XINDEX), then there must be at least one
            // section header in the table whose st_size (resp. st_link) member holds the real
            // number of sections (resp. index of the section string table).
            if (shnum == 0 || shstrndx == SHN_XINDEX) && !array_in_bounds(shoff, size, 1) {
                return invalid_data("File too small for section headers");
            }
            let num = reader.num_sections() as u64;
            if !array_in_bounds(shoff, size, num) {
                return invalid_data("File too small for section headers");
            }

            if (is_64bit && size as usize != mem::size_of::<Elf64_Shdr>()) ||
               (!is_64bit && size as usize != mem::size_of::<Elf32_Shdr>())
            {
                return invalid_data(&format!("Unexpected section header size {} for {}-bit ELF",
                                             size, { if is_64bit { "64" } else { "32" } }));
            }

            // Check that the sections are contained in the file.
            let sections = reader.sections();
            for (index, section) in sections.iter().enumerate() {
                let section_type = section.sh_type();
                if section_type != SHT_NOBITS && section_type != SHT_NULL &&
                   !array_in_bounds(section.sh_offset(), section.sh_size(), 1)
                {
                    return invalid_data(&format!("Section {} not contained within file", index));
                }
            }
        }
        Ok(())
    }

    pub fn little_endian(&self) -> bool {
        self.data[EI_DATA] == ELFDATA2LSB
    }

    pub fn is_64bit(&self) -> bool {
        self.data[EI_CLASS] == ELFCLASS64
    }

    fn file_format_with_extra<E>(&self, extra: E) -> ElfT<(), (), E> {
        match (self.is_64bit(), self.little_endian()) {
            (false, true)  => ElfT::Elf32LE((), extra),
            (false, false) => ElfT::Elf32BE((), extra),
            (true, true)   => ElfT::Elf64LE((), extra),
            (true, false)  => ElfT::Elf64BE((), extra),
        }
    }

    pub fn file_format(&self) -> ElfFormat {
        self.file_format_with_extra(())
    }

    fn slice_ref<T32, T64, E>(&self, offset: usize, num: usize, extra: E) -> ElfSliceRef<'a, T32, T64, E> where
        T32: 'a,
        T64: 'a,
        E: 'a,
    {
        let size = if self.is_64bit() { mem::size_of::<T64>() } else { mem::size_of::<T32>() };
        let end = offset + num * size;
        assert!(end <= self.data.len());
        let ptr = &self.data[offset] as *const u8;
        self.file_format_with_extra(extra)
            .map(move |_| unsafe { slice::from_raw_parts(ptr as *const T32, num) },
                 move |_| unsafe { slice::from_raw_parts(ptr as *const T64, num) })
    }

    pub fn header(&self) -> HeaderRef<'a> {
        let ptr = self.data.as_ptr();
        
        self.file_format().map(
            move |_| unsafe { &*(ptr as *const Elf32_Ehdr) },
            move |_| unsafe { &*(ptr as *const Elf64_Ehdr) })
    }

    pub fn segments(&self) -> SegmentsRef<'a> {
        let ehdr = self.header();
        let phoff = ehdr.e_phoff() as usize;
        let phentsize = ehdr.e_phentsize() as usize;
        let phnum = if phoff == 0 { 0 } else { ehdr.e_phnum() as usize };
        let len = phentsize * phnum;
        let phdr_data = &self.data[phoff..phoff+len];
        SegmentsRef::try_from(ehdr.construct_from(phdr_data, self.data)).unwrap()
    }

    // The ELF file MUST have sections if this file is called.
    fn section_0(&self) -> SectionRef<'a> {
        let ehdr = self.header();
        let shoff = ehdr.e_shoff() as usize;
        let shentsize = ehdr.e_shentsize() as usize;
        debug_assert!(shoff > 0);
        let shdr_data = &self.data[shoff..shoff+shentsize];
        SectionRef::try_from(ehdr.construct_from(shdr_data, self.data)).unwrap()
    }

    pub fn num_sections(&self) -> usize {
        // If e_shoff > 0 and e_shnum() == 0, then section 0's st_size member holds the actual
        // number of sections.
        let ehdr = self.header();
        let shoff = ehdr.e_shoff();
        if shoff == 0 {
            return 0;
        }
        let shnum = ehdr.e_shnum();
        if shnum > 0 {
            shnum as usize
        } else {
            self.section_0().sh_size() as usize
        }
    }

    pub fn section_string_table_index(&self) -> Option<usize> {
        let ehdr = self.header();
        let shstrndx = ehdr.e_shstrndx();
        if shstrndx == SHN_UNDEF {
            return None;
        }
        if shstrndx == SHN_XINDEX {
            Some(self.section_0().sh_link() as usize)
        } else {
            Some(shstrndx as usize)
        }
    }

    pub fn sections(&self) -> SectionsRef<'a> {
        let ehdr = self.header();
        let shoff = ehdr.e_shoff() as usize;
        let shentsize = ehdr.e_shentsize() as usize;
        let shnum = self.num_sections();
        let len = shentsize * shnum;
        let shdr_data = &self.data[shoff..shoff+len];
        SectionsRef::try_from(ehdr.construct_from(shdr_data, self.data)).unwrap()
    }
}


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

            pub fn to_section(self) -> SectionRef<'a> { self.0 }

            pub fn from_section(sec: SectionRef<'a>) -> Option<Self> {
                match sec.sh_type() {
                    $($type)|+ => Some($name(sec)),
                    _          => None,
                }
            }
        }
    }
}

section_type!(SymbolTableSectionRef, SHT_SYMTAB | SHT_DYNSYM);
section_type!(StringTableSectionRef, SHT_STRTAB);
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
