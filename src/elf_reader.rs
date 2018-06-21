#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::io;
use std::iter;
use std::mem;
use std::slice;

use elf_base_types::*;
use elf_types::*;
use elf_constants::*;

#[derive(Debug, Clone)]
pub enum ElfT<T32, T64, E> {
    Elf32LE(T32, E),
    Elf32BE(T32, E),
    Elf64LE(T64, E),
    Elf64BE(T64, E),
}

pub type ElfFileFormat = ElfT<(), (), ()>;

impl<T32, T64, E> ElfT<T32, T64, E> {
    #[inline]
    fn extra(&self) -> &E {
        match *self {
            ElfT::Elf32LE(_, ref e) |
            ElfT::Elf32BE(_, ref e) |
            ElfT::Elf64LE(_, ref e) |
            ElfT::Elf64BE(_, ref e) => &e,
        }
    }

    #[inline]
    fn apply<T, F1, F2>(&self, f1: F1, f2: F2) -> T where
        F1: FnOnce(&T32) -> T,
        F2: FnOnce(&T64) -> T,
    {
        match *self {
            ElfT::Elf32LE(ref x, _) |
            ElfT::Elf32BE(ref x, _) => f1(&x),
            ElfT::Elf64LE(ref x, _) |
            ElfT::Elf64BE(ref x, _) => f2(&x),
        }
    }

    #[inline]
    fn map<R32, R64, F1, F2>(self, f1: F1, f2: F2) -> ElfT<R32, R64, E> where
        F1: FnOnce(T32) -> R32,
        F2: FnOnce(T64) -> R64,
    {
        match self {
            ElfT::Elf32LE(x, e) => ElfT::Elf32LE(f1(x), e),
            ElfT::Elf32BE(x, e) => ElfT::Elf32BE(f1(x), e),
            ElfT::Elf64LE(x, e) => ElfT::Elf64LE(f2(x), e),
            ElfT::Elf64BE(x, e) => ElfT::Elf64BE(f2(x), e),
        }
    }
}

pub type ElfRef<'a, T32, T64, E> = ElfT<&'a T32, &'a T64, E>;
pub type ElfSliceRef<'a, T32, T64, E> = ElfT<&'a [T32], &'a [T64], E>;

impl<'a, T32, T64, E: Clone> ElfSliceRef<'a, T32, T64, E> {
    pub fn len(&self) -> usize {
        self.apply(|s| s.len(), |s| s.len())
    }

    pub fn get(&self, index: usize) -> Option<ElfRef<'a, T32, T64, E>> {
        if index >= self.len() {
            return None;
        }
        Some(self.clone().map(move |slice| &slice[index], move |slice| &slice[index]))
    }

    pub fn iter(&self) -> ElfIter<'a, T32, T64, E> {
        self.clone().map(move |slice| slice.iter(), move |slice| slice.iter())
    }
}

impl<'a, T32, T64, E: 'a + Clone> iter::IntoIterator for ElfSliceRef<'a, T32, T64, E> {
    type Item = ElfRef<'a, T32, T64, E>;
    type IntoIter = ElfIter<'a, T32, T64, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.map(move |slice| slice.iter(), move |slice| slice.iter())
    }
}

pub type ElfIter<'a, T32, T64, E> = ElfT<slice::Iter<'a, T32>, slice::Iter<'a, T64>, E>;

impl<'a, T32, T64, E: Clone> Iterator for ElfIter<'a, T32, T64, E> {
    type Item = ElfRef<'a, T32, T64, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            ElfT::Elf32LE(ref mut it, ref e) => it.next().map(move |x| ElfT::Elf32LE(x, e.clone())),
            ElfT::Elf32BE(ref mut it, ref e) => it.next().map(move |x| ElfT::Elf32BE(x, e.clone())),
            ElfT::Elf64LE(ref mut it, ref e) => it.next().map(move |x| ElfT::Elf64LE(x, e.clone())),
            ElfT::Elf64BE(ref mut it, ref e) => it.next().map(move |x| ElfT::Elf64BE(x, e.clone())),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.apply(|it| it.size_hint(), |it| it.size_hint())
    }
}

impl<'a, T32, T64, E> ExactSizeIterator for ElfIter<'a, T32, T64, E> where
    slice::Iter<'a, T32>: ExactSizeIterator,
    slice::Iter<'a, T64>: ExactSizeIterator,
    E: Clone,
{}

impl<'a, T32, T64, E> iter::FusedIterator for ElfIter<'a, T32, T64, E> where
    slice::Iter<'a, T32>: iter::FusedIterator,
    slice::Iter<'a, T64>: iter::FusedIterator,
    E: Clone,
{}

macro_rules! header_field_impl {
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

    header_field_impl!(e_type,      Elf32_Half, Elf64_Half);
    header_field_impl!(e_machine,   Elf32_Half, Elf64_Half);
    header_field_impl!(e_version,   Elf32_Word, Elf64_Word);
    header_field_impl!(e_entry,     Elf32_Addr, Elf64_Addr);
    header_field_impl!(e_phoff,     Elf32_Off,  Elf64_Off);
    header_field_impl!(e_shoff,     Elf32_Off,  Elf64_Off);
    header_field_impl!(e_flags,     Elf32_Word, Elf64_Word);
    header_field_impl!(e_ehsize,    Elf32_Half, Elf64_Half);
    header_field_impl!(e_phentsize, Elf32_Half, Elf64_Half);
    header_field_impl!(e_phnum,     Elf32_Half, Elf64_Half);
    header_field_impl!(e_shentsize, Elf32_Half, Elf64_Half);
    header_field_impl!(e_shnum,     Elf32_Half, Elf64_Half);
    header_field_impl!(e_shstrndx,  Elf32_Half, Elf64_Half);
}

pub type SegmentRef<'a> = ElfRef<'a, Elf32_Phdr, Elf64_Phdr, &'a [u8]>;
pub type SegmentsRef<'a> = ElfSliceRef<'a, Elf32_Phdr, Elf64_Phdr, &'a [u8]>;

impl<'a> SegmentRef<'a> {
    header_field_impl!(p_type,   Elf32_Word, Elf64_Word);
    header_field_impl!(p_offset, Elf32_Off,  Elf64_Off);
    header_field_impl!(p_vaddr,  Elf32_Addr, Elf64_Addr);
    header_field_impl!(p_paddr,  Elf32_Addr, Elf64_Addr);
    header_field_impl!(p_filesz, Elf32_Word, Elf64_Xword);
    header_field_impl!(p_memsz,  Elf32_Word, Elf64_Xword);
    header_field_impl!(p_flags,  Elf32_Word, Elf64_Word);
    header_field_impl!(p_align,  Elf32_Word, Elf64_Xword);

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
    header_field_impl!(sh_name,      Elf32_Word, Elf64_Word);
    header_field_impl!(sh_type,      Elf32_Word, Elf64_Word);
    header_field_impl!(sh_flags,     Elf32_Word, Elf64_Xword);
    header_field_impl!(sh_addr,      Elf32_Addr, Elf64_Addr);
    header_field_impl!(sh_offset,    Elf32_Off,  Elf64_Off);
    header_field_impl!(sh_size,      Elf32_Word, Elf64_Xword);
    header_field_impl!(sh_link,      Elf32_Word, Elf64_Word);
    header_field_impl!(sh_info,      Elf32_Word, Elf64_Word);
    header_field_impl!(sh_addralign, Elf32_Word, Elf64_Xword);
    header_field_impl!(sh_entsize,   Elf32_Word, Elf64_Xword);

    pub fn section_data(&self) -> &[u8] {
        let section_type = self.sh_type();
        if section_type == SHT_NULL || section_type == SHT_NOBITS {
            return &[][..];
        }
        let offset = self.sh_offset() as usize;
        let size = self.sh_size() as usize;
        let elf_data = self.extra();
        &elf_data[offset..offset+size]
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
        Reader::validate_elf_file(&reader)?;
        Ok(reader)
    }

    fn validate_elf_file(reader: &Reader<'a>) -> io::Result<()> {
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
            let num = hdr.e_shnum() as u64;
            if !array_in_bounds(shoff, size, num) {
                return invalid_data("File too small for section headers");
            }
            if (is_64bit && size as usize != mem::size_of::<Elf64_Shdr>()) ||
               (!is_64bit && size as usize != mem::size_of::<Elf32_Shdr>())
            {
                return invalid_data(&format!("Unexpected section header size {} for {}-bit ELF",
                                             size, { if is_64bit { "64" } else { "32" } }));
            }
            // Check that the sections are contained in the file
            for (index, section) in reader.sections().into_iter().enumerate() {
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

    pub fn file_format(&self) -> ElfFileFormat {
        self.file_format_with_extra(())
    }

    fn slice_ref<T32, T64, E>(&self, offset: usize, num: usize, extra: E) -> ElfSliceRef<'a, T32, T64, E> where
        T32: 'a,
        T64: 'a,
        E: 'a + Clone,
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
        let ptr = self.data as *const _;
        
        self.file_format().map(
            move |_| unsafe { &*(ptr as *const Elf32_Ehdr) },
            move |_| unsafe { &*(ptr as *const Elf64_Ehdr) })
    }

    pub fn segments(&self) -> SegmentsRef<'a> {
        let ehdr = self.header();
        let phoff = ehdr.e_phoff() as usize;
        let phnum = if phoff == 0 { 0 } else { ehdr.e_phnum() as usize };
        self.slice_ref(phoff, phnum, self.data)
    }

    pub fn sections(&self) -> SectionsRef<'a> {
        let ehdr = self.header();
        let shoff = ehdr.e_shoff() as usize;
        let shnum = if shoff == 0 { 0 } else { ehdr.e_shnum() as usize };
        self.slice_ref(shoff, shnum, self.data)
    }
}
