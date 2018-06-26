extern crate elftk;
extern crate failure;

use elftk as elf;

use std::fs;
use std::io;
use std::str;

fn section_flags(flags: elf::Elf_Xword) -> ([u8; 15], usize) {
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

fn print_sections(reader: &elf::Reader) {
    let sections = reader.section_headers();
    let offset = reader.header().e_shoff();
    println!("There are {} section headers, starting at offset 0x{:x}:\n", sections.len(), offset);
    println!("Section Headers:");
    println!("  [Nr] {:17} {:15} {:8} {:6} {:6} ES Flg Lk Inf Al",
             "Name", "Type", "Addr", "Off", "Size");
    for (index, shdr) in sections.into_iter().enumerate() {
        let name = str::from_utf8(reader.section_name(shdr)).unwrap_or("");
        let (flags, len) = section_flags(shdr.sh_flags());
        let flags = str::from_utf8(&flags[..len]).unwrap();
        println!("  [{:2}] {:17} {:15} {:08x} {:06x} {:06x} {:02x} {:>3} {:>2} {:>3} {:>2}",
                 index, name, elf::section_type_name(shdr.sh_type()), shdr.sh_addr(),
                 shdr.sh_offset(), shdr.sh_size(), shdr.sh_entsize(), &flags[..len],
                 shdr.sh_link(), shdr.sh_info(), shdr.sh_addralign());
    }
    println!("Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  p (processor specific)");
}

use failure::Error;
fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} input", &args[0]);
        Err(io::Error::new(io::ErrorKind::InvalidInput, "needs arg"))?;
    }
    let data = fs::read(&args[1])?;
    let reader = elf::Reader::new(&data)?;
    print_sections(&reader);
    Ok(())
}
