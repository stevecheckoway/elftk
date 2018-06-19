#![allow(non_camel_case_types)]
#![allow(dead_code)]

// Re-export these.
pub use elf_base_types::*;
pub use elf_types::*;
pub use elf_constants::*;
pub use elf_reader::Reader;

// use std::io::{self, Read, Write};
// use std::mem;
// use std::slice;

/*
fn invalid_data<T> (msg: &str) -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::InvalidData, msg))
}

trait ReadInto : Read + Sized {
    fn read_into(&mut self, count: usize, buf: &mut Vec<u8>) -> io::Result<()> {
        let reader_ref = self.by_ref();
        reader_ref.take(count as u64)
            .read_to_end(buf)
            .and_then(|size| {
                if size == count {
                    Ok(())
                } else {
                    invalid_data(&format!("Couldn't read {} bytes, read {}",
                                          count, size))
                }
            })
    }

    fn read_until_count_into(&mut self, count: usize, buf: &mut Vec<u8>) -> io::Result<()> {
        let len = buf.len();
        if len >= count {
            return Ok(());
        }
        self.read_into(count - len, buf)
    }
}

impl<T: Read> ReadInto for T {}

pub struct Reader {
    data: Vec<u8>,
}

impl Reader {
    pub fn new<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut data: Vec<u8> = Vec::new();
        reader.read_into(EI_NIDENT, &mut data)?;
        if &data[0..4] != b"\x7fELF" {
            return invalid_data("Not an ELF file");
        }
        if data[EI_VERSION] != EV_CURRENT {
            return invalid_data(&format!("Unknown ELF header version {}", data[EI_VERSION]));
        }
        if data[EI_CLASS] != 1 {
            return invalid_data("Not an ELF32 file");
        }
        if data[EI_DATA] != ELFDATA2LSB {
            return invalid_data("Not a little endian ELF file");
        }

        // Read the rest of the header.
        reader.read_until_count_into(mem::size_of::<Elf32_Ehdr>(), &mut data)?;

        // Construct the result and read the rest of the data.
        let mut result = Reader { data: data };
        let total_size = {
            let hdr = result.header();
            if hdr.e_shoff == 0 {
                return invalid_data("Missing section header");
            }
            hdr.e_shoff as usize + hdr.e_shentsize as usize * hdr.e_shnum as usize
        };
        reader.read_until_count_into(total_size, &mut result.data)?;
        Ok(result)
    }

    pub fn header(&self) -> &Elf32_Ehdr {
        let ptr = self.data.as_slice() as *const _;
        unsafe {
            &*(ptr as *const Elf32_Ehdr)
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn sections(&self) -> &[Elf32_Shdr] {
        let hdr = self.header();
        let start = hdr.e_shoff as usize;
        let num = hdr.e_shnum as usize; // XXX: This value can be stored in the first section header
        let size = hdr.e_shentsize as usize * num;
        let shdr_table_ptr = &self.data[start..start+size] as *const _;
        unsafe {
            slice::from_raw_parts(shdr_table_ptr as *const Elf32_Shdr, num)
        }
    }
}

pub struct Writer<'a> {
    hdr: Elf64_Ehdr,
    sections: Vec<(Elf64_Shdr, &'a [u8])>,
}

fn align_to(x: u64, align: u64) -> u64 {
    if align == 0 {
        return x;
    }
    let mask = align - 1;
    (x + mask) & !mask
}

impl<'a> Writer<'a> {
    pub fn new(hdr: Elf64_Ehdr) -> Self {
        Writer {
            hdr: hdr,
            sections: Vec::new(),
        }
    }

    pub fn add_section<'b>(&'b mut self, hdr: Elf64_Shdr, data: &'a [u8]) 
        where 'a: 'b
    {
        self.sections.push((hdr, data));
    }

    pub fn write<W: Write>(mut self, writer: &mut W) -> io::Result<()> {
        // Compute the layout.
        // 1. Elf64_Ehdr
        // 2. No program header table, but it'd go here if we had one.
        // 3. Sections 0 through n-1, appropriately aligned.
        // 4. Section header table.
        // We need to compute the offset of the section header table.
        let mut offset: u64 = mem::size_of::<Elf64_Ehdr>() as u64;
        for (shdr, _data) in &mut self.sections {
            if shdr.sh_type == SHT_NULL || shdr.sh_type == SHT_NOBITS {
                continue;
            }
            offset = align_to(offset, shdr.sh_addralign as u64);
            shdr.sh_offset = offset;
            offset += shdr.sh_size as u64;
        }
        offset = align_to(offset, 8);
        self.hdr.e_shoff = offset;
        self.hdr.e_shnum = self.sections.len() as Elf64_Half;

        // 1. Write the header
        let hdr_slice = unsafe {
            slice::from_raw_parts(&self.hdr as *const _ as *const u8,
                                  mem::size_of::<Elf64_Ehdr>())
        };
        writer.write_all(hdr_slice)?;
        offset = mem::size_of::<Elf64_Ehdr>() as u64;
        // 2. Nothing to do for the program header.
        // 3. Write the sections.
        for (shdr, data) in &self.sections {
            if shdr.sh_type == SHT_NULL || shdr.sh_type == SHT_NOBITS {
                continue;
            }
            let padding = align_to(offset, shdr.sh_addralign as u64) - offset;
            for _ in 0..padding {
                writer.write_all(&[0])?;
            }
            writer.write_all(data)?;
            offset += padding + data.len() as u64;
        }
        // 4. Write the section header table.
        let padding = align_to(offset, 8) - offset;
        for _ in 0..padding {
            writer.write_all(&[0])?;
        }
        for (shdr, _) in &self.sections {
            let shdr_slice = unsafe {
                slice::from_raw_parts(shdr as *const Elf64_Shdr as *const u8,
                                      mem::size_of::<Elf64_Shdr>())
            };
            println!("{:?}", shdr);
            writer.write_all(shdr_slice)?;
        }
        Ok(())
    }
}
*/
