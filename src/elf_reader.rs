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
pub enum ElfStructRef<'a, T32, T64, E = ()> where
    T32: 'a,
    T64: 'a,
    E: 'a + Clone,
{
    Elf32LE(&'a T32, E),
    Elf32BE(&'a T32, E),
    Elf64LE(&'a T64, E),
    Elf64BE(&'a T64, E),
}

#[derive(Debug, Clone)]
pub enum ElfStructSliceRef<'a, T32, T64, E = ()> where
    T32: 'a,
    T64: 'a,
    E: 'a + Clone,
{
    Elf32LE(&'a [T32], E),
    Elf32BE(&'a [T32], E),
    Elf64LE(&'a [T64], E),
    Elf64BE(&'a [T64], E),
}

impl<'a, T32, T64, E: Clone> ElfStructSliceRef<'a, T32, T64, E> {
    pub fn len(&self) -> usize {
        match *self {
            ElfStructSliceRef::Elf32LE(slice, _) => slice.len(),
            ElfStructSliceRef::Elf32BE(slice, _) => slice.len(),
            ElfStructSliceRef::Elf64LE(slice, _) => slice.len(),
            ElfStructSliceRef::Elf64BE(slice, _) => slice.len(),
        }
    }

    pub fn get(&self, index: usize) -> Option<ElfStructRef<'a, T32, T64, E>> {
        if index >= self.len() {
            return None;
        }
        Some(match *self {
            ElfStructSliceRef::Elf32LE(slice, ref e) => ElfStructRef::Elf32LE(&slice[index], e.clone()),
            ElfStructSliceRef::Elf32BE(slice, ref e) => ElfStructRef::Elf32BE(&slice[index], e.clone()),
            ElfStructSliceRef::Elf64LE(slice, ref e) => ElfStructRef::Elf64LE(&slice[index], e.clone()),
            ElfStructSliceRef::Elf64BE(slice, ref e) => ElfStructRef::Elf64BE(&slice[index], e.clone()),
        })
    }

    pub fn iter(&self) -> ElfStructIter<'a, T32, T64, E> {
        ElfStructIter {
            index: 0,
            slice: match *self {
                ElfStructSliceRef::Elf32LE(slice, ref e) => ElfStructSliceRef::Elf32LE(slice.clone(), e.clone()),
                ElfStructSliceRef::Elf32BE(slice, ref e) => ElfStructSliceRef::Elf32BE(slice.clone(), e.clone()),
                ElfStructSliceRef::Elf64LE(slice, ref e) => ElfStructSliceRef::Elf64LE(slice.clone(), e.clone()),
                ElfStructSliceRef::Elf64BE(slice, ref e) => ElfStructSliceRef::Elf64BE(slice.clone(), e.clone()),
            },
        }
    }
}

impl<'a, T32, T64, E: Clone> iter::IntoIterator for ElfStructSliceRef<'a, T32, T64, E> {
    type Item = ElfStructRef<'a, T32, T64, E>;
    type IntoIter = ElfStructIter<'a, T32, T64, E>;

    fn into_iter(self) -> Self::IntoIter {
        ElfStructIter { index: 0, slice: self }
    }
}

#[derive(Debug, Clone)]
pub struct ElfStructIter<'a, T32, T64, E> where
    T32: 'a,
    T64: 'a,
    E: 'a + Clone,
{
    index: usize,
    slice: ElfStructSliceRef<'a, T32, T64, E>,
}

impl<'a, T32, T64, E: Clone> Iterator for ElfStructIter<'a, T32, T64, E> {
    type Item = ElfStructRef<'a, T32, T64, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.slice.get(self.index);
        if result.is_some() {
            self.index += 1;
        }
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.slice.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a, T32, T64, E: Clone> ExactSizeIterator for ElfStructIter<'a, T32, T64, E> {
    fn len(&self) -> usize {
        self.slice.len() - self.index
    }
}

impl<'a, T32, T64, E: Clone> iter::FusedIterator for ElfStructIter<'a, T32, T64, E> { }

macro_rules! header_field_impl {
    ( $field:ident, $t32:ident, $t64:ident ) => {
        pub fn $field(&self) -> $t64 {
            match *self {
                ElfStructRef::Elf32LE(hdr, _) => $t32::from_le(hdr.$field) as $t64,
                ElfStructRef::Elf32BE(hdr, _) => $t32::from_be(hdr.$field) as $t64,
                ElfStructRef::Elf64LE(hdr, _) => $t64::from_le(hdr.$field),
                ElfStructRef::Elf64BE(hdr, _) => $t64::from_be(hdr.$field),
            }
        }
    }
}

pub type HeaderRef<'a> = ElfStructRef<'a, Elf32_Ehdr, Elf64_Ehdr>;

impl<'a> HeaderRef<'a> {
    pub fn e_ident(&self) -> &[u8; EI_NIDENT] {
        match *self {
            ElfStructRef::Elf32LE(hdr, _) => &hdr.e_ident,
            ElfStructRef::Elf32BE(hdr, _) => &hdr.e_ident,
            ElfStructRef::Elf64LE(hdr, _) => &hdr.e_ident,
            ElfStructRef::Elf64BE(hdr, _) => &hdr.e_ident,
        }
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

pub type SegmentRef<'a> = ElfStructRef<'a, Elf32_Phdr, Elf64_Phdr, &'a [u8]>;
pub type SegmentsRef<'a> = ElfStructSliceRef<'a, Elf32_Phdr, Elf64_Phdr, &'a [u8]>;

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
        let elf_data = match *self {
            ElfStructRef::Elf32LE(_, data) |
            ElfStructRef::Elf32BE(_, data) |
            ElfStructRef::Elf64LE(_, data) |
            ElfStructRef::Elf64BE(_, data) => data
        };
        &elf_data[offset..offset+size]
    }
}

pub type SectionRef<'a> = ElfStructRef<'a, Elf32_Shdr, Elf64_Shdr, &'a [u8]>;
pub type SectionsRef<'a> = ElfStructSliceRef<'a, Elf32_Shdr, Elf64_Shdr, &'a [u8]>;

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
        let elf_data = match *self {
            ElfStructRef::Elf32LE(_, data) |
            ElfStructRef::Elf32BE(_, data) |
            ElfStructRef::Elf64LE(_, data) |
            ElfStructRef::Elf64BE(_, data) => data
        };
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
                .map_or(false, |end| end <= len)
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

    fn slice_ref<T32, T64, E>(&self, offset: usize, num: usize, extra: E) -> ElfStructSliceRef<'a, T32, T64, E> where
        T32: 'a,
        T64: 'a,
        E: 'a + Clone,
    {
        let size = if self.is_64bit() { mem::size_of::<T32>() } else { mem::size_of::<T64>() };
        let end = offset + num * size;
        assert!(end <= self.data.len());
        let ptr = &self.data[offset] as *const u8;
        match (self.is_64bit(), self.little_endian()) {
            (false, true)  => ElfStructSliceRef::Elf32LE(unsafe { slice::from_raw_parts(ptr as *const T32, num) }, extra),
            (false, false) => ElfStructSliceRef::Elf32BE(unsafe { slice::from_raw_parts(ptr as *const T32, num) }, extra),
            (true, true)   => ElfStructSliceRef::Elf64LE(unsafe { slice::from_raw_parts(ptr as *const T64, num) }, extra),
            (true, false)  => ElfStructSliceRef::Elf64BE(unsafe { slice::from_raw_parts(ptr as *const T64, num) }, extra),
        }
    }

    pub fn header(&self) -> HeaderRef<'a> {
        let ptr = self.data as *const _;
        
        match (self.is_64bit(), self.little_endian()) {
            (false, true)  => ElfStructRef::Elf32LE(unsafe { &*(ptr as *const Elf32_Ehdr) }, ()),
            (false, false) => ElfStructRef::Elf32BE(unsafe { &*(ptr as *const Elf32_Ehdr) }, ()),
            (true, true)   => ElfStructRef::Elf64LE(unsafe { &*(ptr as *const Elf64_Ehdr) }, ()),
            (true, false)  => ElfStructRef::Elf64BE(unsafe { &*(ptr as *const Elf64_Ehdr) }, ()),
        }
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
