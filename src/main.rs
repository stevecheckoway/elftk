mod elf;
mod elf_base_types;
mod elf_constants;
mod elf_types;
mod elf_reader;


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
    let sections = reader.sections();
    let offset = reader.header().e_shoff();
    println!("There are {} section headers, starting at offset 0x{:x}:\n", sections.len(), offset);
    println!("Section Headers:");
    println!("  [Nr] {:17} {:15} {:8} {:6} {:6} ES Flg Lk Inf Al",
             "Name", "Type", "Addr", "Off", "Size");
    for (index, section) in sections.into_iter().enumerate() {
        let name = str::from_utf8(section.section_name()).unwrap_or("");
        let (flags, len) = section_flags(section.sh_flags());
        let flags = str::from_utf8(&flags[..len]).unwrap();
        println!("  [{:2}] {:17} {:15} {:08x} {:06x} {:06x} {:02x} {:>3} {:>2} {:>3} {:>2}",
                 index, name, elf::section_type_name(section.sh_type()), section.sh_addr(),
                 section.sh_offset(), section.sh_size(), section.sh_entsize(), &flags[..len],
                 section.sh_link(), section.sh_info(), section.sh_addralign());
    }
    println!("Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  p (processor specific)");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} input", &args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "needs arg"));
    }
    let data = fs::read(&args[1])?;
    let reader = elf::Reader::new(&data)?;
    print_sections(&reader);

    //let hdr = reader.header();
    //let mut new_hdr = elf::Elf64_Ehdr::new();
    //new_hdr.e_ident[elf::EI_OSABI] = hdr.e_ident[elf::EI_OSABI];
    //new_hdr.e_ident[elf::EI_ABIVERSION] = hdr.e_ident[elf::EI_ABIVERSION];
    //new_hdr.e_type = hdr.e_type;
    //new_hdr.e_machine = elf::EM_X86_64;
    //new_hdr.e_entry = hdr.e_entry as elf::Elf64_Addr;
    //new_hdr.e_shstrndx = hdr.e_shstrndx;
    //let mut writer = elf::Writer::new(new_hdr);

    //let mut count = 0;
    //for shdr in reader.sections() {
    //    count = count + 1;
    //    let mut new_shdr = elf::Elf64_Shdr {
    //        sh_name: shdr.sh_name,
    //        sh_type: shdr.sh_type,
    //        sh_flags: shdr.sh_flags as elf::Elf64_Xword,
    //        sh_addr: shdr.sh_addr as elf::Elf64_Addr,
    //        sh_offset: shdr.sh_offset as elf::Elf64_Off,
    //        sh_size: shdr.sh_size as elf::Elf64_Xword,
    //        sh_link: shdr.sh_link,
    //        sh_info: shdr.sh_info,
    //        sh_addralign: shdr.sh_addralign as elf::Elf64_Xword,
    //        sh_entsize: shdr.sh_entsize as elf::Elf64_Xword,
    //    };
    //    if shdr.sh_type == elf::SHT_RELA {
    //        println!("Found RELA section");
    //    } else if shdr.sh_type == elf::SHT_REL {
    //        println!("Found REL section");
    //    }
    //    let data = if shdr.sh_type == elf::SHT_NULL || shdr.sh_type == elf::SHT_NOBITS {
    //            &[]
    //        } else {
    //            let data_start = shdr.sh_offset as usize;
    //            let data_end = data_start + shdr.sh_size as usize;
    //            &reader.data()[data_start..data_end]
    //        };
    //    writer.add_section(new_shdr, data);
    //}
    //let mut output = File::create(&args[2])?;
    //writer.write(&mut output)?;
    Ok(())
}
