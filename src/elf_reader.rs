#![allow(non_camel_case_types)]
#![allow(dead_code)]

use elf_base_types::*;
use elf_types::*;
use elf_constants::*;

use std::mem;
use std::io;

pub enum HeaderRef<'a> {
    Elf32(&'a Elf32_Ehdr),
    Elf64(&'a Elf64_Ehdr),
}

macro_rules! header_field_impl {
    ( $field:ident, $t32:ident, $t64:ident ) => {
        pub fn $field(&self) -> $t64 {
            match *self {
                HeaderRef::Elf32(hdr) => {
                    if hdr.e_ident[EI_DATA] == ELFDATA2LSB {
                        $t32::from_le(hdr.$field) as $t64
                    } else {
                        $t32::from_be(hdr.$field) as $t64
                    }
                },
                HeaderRef::Elf64(hdr) => {
                    if hdr.e_ident[EI_DATA] == ELFDATA2LSB {
                        $t64::from_le(hdr.$field)
                    } else {
                        $t64::from_be(hdr.$field)
                    }
                },
            }
        }
    }
}

impl<'a> HeaderRef<'a> {
    pub fn e_ident(&self) -> &[u8; EI_NIDENT] {
        match *self {
            HeaderRef::Elf32(hdr) => &hdr.e_ident,
            HeaderRef::Elf64(hdr) => &hdr.e_ident,
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

// Section field is true for little endian.
pub enum ProgramHeaderRef<'a> {
    Elf32(&'a Elf32_Phdr, bool),
    Elf64(&'a Elf64_Phdr, bool),
}

macro_rules! pheader_field_impl {
    ( $field:ident, $t32:ident, $t64:ident ) => {
        pub fn $field(&self) -> $t64 {
            match self {
                ProgramHeaderRef::Elf32(hdr, true)  => $t32::from_le(hdr.$field) as $t64,
                ProgramHeaderRef::Elf32(hdr, false) => $t32::from_be(hdr.$field) as $t64,
                ProgramHeaderRef::Elf64(hdr, true)  => $t64::from_le(hdr.$field),
                ProgramHeaderRef::Elf64(hdr, false) => $t64::from_be(hdr.$field),
            }
        }
    }
}

impl<'a> ProgramHeaderRef<'a> {
    pheader_field_impl!(p_type,   Elf32_Word, Elf64_Word);
    pheader_field_impl!(p_offset, Elf32_Off,  Elf64_Off);
    pheader_field_impl!(p_vaddr,  Elf32_Addr, Elf64_Addr);
    pheader_field_impl!(p_paddr,  Elf32_Addr, Elf64_Addr);
    pheader_field_impl!(p_filesz, Elf32_Word, Elf64_Word);
    pheader_field_impl!(p_memsz,  Elf32_Word, Elf64_Word);
    pheader_field_impl!(p_flags,  Elf32_Word, Elf64_Word);
    pheader_field_impl!(p_align,  Elf32_Word, Elf64_Word);
}

pub struct SegmentRef<'a> {
    pub header: ProgramHeaderRef<'a>,
    pub data: &'a [u8],
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
        let hdr = reader.header();
        let len = data.len() as u64;
        let is_64bit = reader.is_64bit();

        if len < hdr.e_entry() {
            return invalid_data("File too small for entry point");
        }
        let phoff = hdr.e_phoff();
        if phoff > 0 {
            let size = hdr.e_phentsize() as u64;
            let num = hdr.e_phnum() as u64;
            if len < phoff + size * num {
                return invalid_data("File too small for program headers");
            }
            if (is_64bit && size as usize != mem::size_of::<Elf64_Phdr>()) ||
               (!is_64bit && size as usize != mem::size_of::<Elf32_Phdr>())
            {
                return invalid_data(&format!("Unexpected program header size {} for {}-bit ELF",
                                             size, { if is_64bit { "64" } else { "32" } }));
            }
            // XXX: Check that the program headers are contained in the file
        }
        let shoff = hdr.e_shoff();
        if shoff > 0 {
            let size = hdr.e_shentsize() as u64;
            let num = hdr.e_shnum() as u64;
            if len < shoff + size * num {
                return invalid_data("file too small for section headers");
            }
            if (is_64bit && size as usize != mem::size_of::<Elf64_Shdr>()) ||
               (!is_64bit && size as usize != mem::size_of::<Elf32_Shdr>())
            {
                return invalid_data(&format!("Unexpected section header size {} for {}-bit ELF",
                                             size, { if is_64bit { "64" } else { "32" } }));
            }
            // XXX: Check that the section headers are contained in the file
        }
        Ok(reader)
    }

    pub fn little_endian(&self) -> bool {
        self.data[EI_DATA] == ELFDATA2LSB
    }

    pub fn is_64bit(&self) -> bool {
        self.data[EI_CLASS] == ELFCLASS64
    }

    pub fn header(&self) -> HeaderRef<'a> {
        let ptr = self.data as *const _;
        if self.is_64bit() {
            HeaderRef::Elf64(unsafe { &*(ptr as *const Elf64_Ehdr) })
        } else {
            HeaderRef::Elf32(unsafe { &*(ptr as *const Elf32_Ehdr) })
        }
    }

    pub fn program_header(&self, idx: usize) -> ProgramHeaderRef<'a> {
        let ehdr = self.header();
        let phoff = ehdr.e_phoff() as usize;
        let phnum = ehdr.e_phnum() as usize;
        assert!(phoff > 0 && idx < phnum, "Tried to access an out of range program header");
        if self.is_64bit() {
            let ptr = &self.data[phoff + idx*mem::size_of::<Elf64_Phdr>()] as *const _;
            let phdr = unsafe { &*(ptr as *const Elf64_Phdr) };
            ProgramHeaderRef::Elf64(phdr, self.little_endian())
        } else {
            let ptr = &self.data[phoff + idx*mem::size_of::<Elf32_Phdr>()] as *const _;
            let phdr = unsafe { &*(ptr as *const Elf32_Phdr) };
            ProgramHeaderRef::Elf32(phdr, self.little_endian())
        }
    }

    pub fn segment(&self, idx: usize) -> SegmentRef<'a> {
        let hdr = self.program_header(idx);
        let offset = hdr.p_offset() as usize;
        let size = hdr.p_filesz() as usize;
        SegmentRef {
            header: hdr,
            data: &self.data[offset..offset+size],
        }
    }

    pub fn segments<'b>(&'b self) -> SegmentIter<'a, 'b> where
        'a: 'b
    {
        SegmentIter {
            reader: self,
            idx: 0,
        }
    }
}

pub struct SegmentIter<'a, 'b> where
    'a: 'b
{
    reader: &'b Reader<'a>,
    idx: Elf_Half,
}

impl<'a, 'b> Iterator for SegmentIter<'a, 'b> {
    type Item = SegmentRef<'a>;

    fn next(&mut self) -> Option<SegmentRef<'a>> {
        let seg = self.reader.segment(self.idx as usize);
        self.idx += 1;
        Some(seg)
    }
}

