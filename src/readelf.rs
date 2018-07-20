#[macro_use]
extern crate clap;
extern crate elftk;
extern crate failure;


use std::fs;
use std::str;

use clap::{Arg, AppSettings};
use elftk as elf;

mod names;
use names::*;

use failure::Error;

fn to_utf8(s: &[u8]) -> &str {
    str::from_utf8(s).unwrap_or("<invalid>")
}

fn entries(num: usize) -> &'static str {
    if num == 1 { "entry" } else { "entries" }
}

fn print_header(reader: &elf::Reader) {
    let ehdr = reader.header();
    let e_ident = ehdr.e_ident();

    println!("ELF Header:");
    print!("  Magic:  ");
    for i in &e_ident {
        print!(" {:02x}", i);
    }
    println!("\n  {:34} {}", "Class", elf::elf_class_name(e_ident[elf::EI_CLASS]));
    let data = match e_ident[elf::EI_DATA] {
        elf::ELFDATANONE => "None",
        elf::ELFDATA2LSB => "2's complement, little endian",
        elf::ELFDATA2MSB => "2's complement, big endian",
        _                => "Unknown",
    };
    println!("  {:34} {}", "Data", data);
    println!("  {:34} {}{}", "Version", e_ident[elf::EI_VERSION],
             if elf::Elf_Word::from(e_ident[elf::EI_VERSION]) == elf::EV_CURRENT { " (current)" } else { "" });
    println!("  {:34} {}", "OS/ABI", elf::elf_osabi_name(e_ident[elf::EI_OSABI]));
    println!("  {:34} {}", "ABI Version", e_ident[elf::EI_ABIVERSION]);
    let file_type = match ehdr.e_type() {
        elf::ET_REL  => " (Relocatable file)",
        elf::ET_EXEC => " (Executable file)",
        elf::ET_DYN  => " (Shared object file)",
        elf::ET_CORE => " (Core file)",
        _            => "",
    };
    println!("  {:34} {}{}", "Type", elf::elf_type_name(ehdr.e_type()), file_type);
    println!("  {:34} {}", "Machine", elf::elf_machine_name(ehdr.e_machine()));
    println!("  {:34} 0x{:x}", "Version", ehdr.e_version());
    println!("  {:34} 0x{:x}", "Entry point address", ehdr.e_entry());
    println!("  {:34} {} (bytes into file)", "Start of program headers", ehdr.e_phoff());
    println!("  {:34} {} (bytes into file)", "Start of section headers", ehdr.e_shoff());
    println!("  {:34} 0x{:x}", "Flags", ehdr.e_flags());
    println!("  {:34} {} (bytes)", "Size of this header", ehdr.e_ehsize());
    println!("  {:34} {} (bytes)", "Size of program headers", ehdr.e_phentsize());
    println!("  {:34} {}", "Number of program headers", ehdr.e_phnum());
    println!("  {:34} {} (bytes)", "Size of section headers", ehdr.e_shentsize());
    println!("  {:34} {}", "Number of section headers", ehdr.e_shnum());
    println!("  {:34} {}", "Section header string table index",
             reader.section_string_table_index().unwrap_or(0));
}

fn print_sections(reader: &elf::Reader, show_count: bool) {
    let sections = reader.section_headers();
    let offset = reader.header().e_shoff();
    if show_count {
        println!("There are {} section headers, starting at offset 0x{:x}:", sections.len(), offset);
    }
    println!("\nSection Headers:");
    println!("  [Nr] {:17} {:15} {:8} {:6} {:6} ES Flg Lk Inf Al",
             "Name", "Type", "Addr", "Off", "Size");
    for (index, shdr) in sections.into_iter().enumerate() {
        let name = to_utf8(reader.section_name(shdr));
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

fn symbol_name<'a>(reader: &elf::Reader<'a>, sym: &elf::SymbolRef<'a>) -> &'a str {
    if sym.symbol_type() == elf::STT_SECTION {
        if let elf::SectionIndex::Normal(index) = sym.section {
            reader.section_headers().get(index as usize)
                .map(|shdr| reader.section_name(shdr))
                .ok()
        } else {
            None
        }
    } else {
        sym.symbol_name
    }.map_or("", to_utf8)
}

fn print_symbols(reader: &elf::Reader, dynamic: bool) -> Result<(), Error> {
    let symtab_section = if dynamic { reader.dynsym()? } else { reader.symtab()? };
    let symtab_section = if let Some(s) = symtab_section {
        s
    } else {
        return Ok(());
    };
    let symtab = if let elf::SectionDataRef::SymbolTable(s) = symtab_section.data { s } else { unreachable!() };
    let section_name = symtab_section.name.map_or("", to_utf8);
    println!("\nSymbol table '{}' contains {} {}:", section_name, symtab.len(), entries(symtab.len()));
    println!("   Num:    Value  Size Type    Bind   Vis      Ndx Name");
    for i in 0 .. symtab.len() {
        let symbol = symtab.get(i)?;
        // This would be sane. But readelf doesn't do that.
        // let name = symbol_name(reader, &symbol);
        let name = symbol.symbol_name.map_or("", to_utf8);
        println!("{:6}: {:08x} {:5} {:<7} {:<6} {:<6} {:>3} {}",
                 i, symbol.value, symbol.size,
                 symbol_type(symbol.symbol_type()), symbol_binding(symbol.binding()),
                 symbol_visibility(symbol.visibility()), symbol_index(symbol.section), name);
    }

    Ok(())
}

fn print_relocations(reader: &elf::Reader) -> Result<(), Error> {
    let machine = reader.header().e_machine();
    for shdr in reader.section_headers() {
        let section = match shdr.sh_type() {
            elf::SHT_REL | elf::SHT_RELA => reader.get_section(shdr)?,
            _        => continue,
        };
        match section.data {
            elf::SectionDataRef::RelocationTable(tab) => {
                // 32-bit
                println!("\nRelocation section '{}' at offset 0x{:x} contains {} {}:",
                         section.name.map_or("", to_utf8), section.shdr.sh_offset(),
                         tab.entries.len(), entries(tab.entries.len()));
                println!(" Offset     Info    Type            Sym.Value  Sym. Name");
                let symtab = tab.symbol_table;
                for entry in tab.entries {
                    let symbol = symtab.get(entry.symbol_index() as usize)?;
                    let name = symbol_name(reader, &symbol);
                    println!("{:08x}  {:08x} {:16}  {:08x}   {}",
                             entry.r_offset(), entry.r_info(),
                             relocation_name(machine, entry.relocation_type()),
                             symbol.value, name);
                }
            },
            elf::SectionDataRef::ExplicitRelocationTable(tab) => {
                // 64-bit
                println!("\nRelocation section '{}' at offset 0x{:x} contains {} {}:",
                         section.name.map_or("", to_utf8), section.shdr.sh_offset(),
                         tab.entries.len(), entries(tab.entries.len()));
                println!("  Offset          Info           Type           Sym. Value    Sym. Name + Addend");
                let symtab = tab.symbol_table;
                for entry in tab.entries {
                    let symbol = symtab.get(entry.symbol_index() as usize)?;
                    let name = symbol_name(reader, &symbol);
                    let addend = entry.r_addend();
                    let (addend, sign) = if addend < 0 { (-addend, '-') } else { (addend, '+') };
                    println!("{:012x}  {:012x} {:16}  {:016x} {} {} {}",
                             entry.r_offset(), entry.r_info(),
                             relocation_name(machine, entry.relocation_type()),
                             symbol.value, name, sign, addend);
                }
            },
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let matches = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .arg(Arg::with_name("all")
             .help("Equivalent to: -h -l -S -s -r -d -V -A -I")
             .short("a")
             .long("all"))
        .arg(Arg::with_name("file-header")
             .help("Display the ELF file header")
             .short("h")
             .long("file-header"))
        .arg(Arg::with_name("program-headers")
             .help("Display the program headers")
             .short("l")
             .long("program-headers"))
        .arg(Arg::with_name("segments")
             .help("An alias for --program-headers")
             .long("segments"))
        .arg(Arg::with_name("section-headers")
             .help("Display the section headers")
             .short("S")
             .long("sections"))
        .arg(Arg::with_name("headers")
             .help("Equivalent to: -h -l -S")
             .short("e")
             .long("headers"))
        .arg(Arg::with_name("syms")
             .help("Display the symbol table")
             .short("s")
             .long("syms"))
        .arg(Arg::with_name("symbols")
             .help("An alias for --syms")
             .long("symbols"))
        .arg(Arg::with_name("dyn-syms")
             .help("Display the dynamic symbol table")
             .long("dyn-syms"))
        .arg(Arg::with_name("relocs")
             .help("Display the relocations (if present)")
             .short("r")
             .long("relocs"))
        .arg(Arg::with_name("elf-file")
             .help("Input ELF files")
             .multiple(true)
             .required(true))
        .help_short("H")
        .version_short("v")
        .get_matches();

    let all = matches.is_present("all");
    let headers = all || matches.is_present("headers");
    let file_header = headers || matches.is_present("file-header");
    let _program_headers = headers || matches.is_present("program-headers") || matches.is_present("segments");
    let section_headers = headers || matches.is_present("section-headers") || matches.is_present("sections");
    let symbols = all || matches.is_present("syms") || matches.is_present("symbols");
    let dynsyms = matches.is_present("dyn-sym");
    let relocations = all || matches.is_present("relocs");
    let input = matches.values_of("elf-file").unwrap();

    for file in input {
        let data = fs::read(file)?;
        let reader = elf::Reader::new(&data)?;
        if file_header {
            print_header(&reader);
        }
        if section_headers {
            print_sections(&reader, !file_header);
        }
        // TODO: program_headers
        // TODO: dynamic
        if relocations {
            print_relocations(&reader)?;
        }
        if dynsyms {
            print_symbols(&reader, true)?;
        }
        if symbols {
            print_symbols(&reader, false)?;
        }
        // TODO: histogram
    }
    Ok(())
}
