mod elf;
mod elf_base_types;
mod elf_constants;
mod elf_types;
mod elf_reader;


use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} input output", &args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "needs arg"));
    }
    let data = fs::read(&args[1])?;
    let reader = elf::Reader::new(&data)?;
    let hdr = reader.header();
    println!("{:?}", hdr);

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
