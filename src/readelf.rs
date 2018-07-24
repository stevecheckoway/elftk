#[macro_use]
extern crate clap;
extern crate elftk;
extern crate failure;


use std::cmp::{max, min};
use std::fs;
use std::result;
use std::str;

use clap::{Arg, AppSettings};
use elftk as elf;

mod names;
use names::*;

use failure::Error;

type Result<T> = result::Result<T, Error>;

fn to_utf8(s: &[u8]) -> &str {
    str::from_utf8(s).unwrap_or("<invalid>")
}

fn entries(num: usize) -> &'static str {
    if num == 1 { "entry" } else { "entries" }
}

fn file_type(ftype: elf::Elf_Half) -> &'static str {
    match ftype {
        elf::ET_REL  => "REL (Relocatable file)",
        elf::ET_EXEC => "EXEC (Executable file)",
        elf::ET_DYN  => "DYN (Shared object file)",
        elf::ET_CORE => "CORE (Core file)",
        _            => "<Unknown>",
    }
}


fn print_header(reader: &elf::Reader) {
    let ehdr = reader.elf_header();
    let e_ident = ehdr.e_ident();

    println!("ELF Header:");
    print!("  Magic:  ");
    for i in &e_ident {
        print!(" {:02x}", i);
    }
    println!("\n  {:34} {}", "Class", elf::class_name(e_ident[elf::EI_CLASS]));
    let data = match e_ident[elf::EI_DATA] {
        elf::ELFDATANONE => "None",
        elf::ELFDATA2LSB => "2's complement, little endian",
        elf::ELFDATA2MSB => "2's complement, big endian",
        _                => "Unknown",
    };
    println!("  {:34} {}", "Data", data);
    println!("  {:34} {}{}", "Version", e_ident[elf::EI_VERSION],
             if elf::Elf_Word::from(e_ident[elf::EI_VERSION]) == elf::EV_CURRENT { " (current)" } else { "" });
    println!("  {:34} {}", "OS/ABI", elf::osabi_name(e_ident[elf::EI_OSABI]));
    println!("  {:34} {}", "ABI Version", e_ident[elf::EI_ABIVERSION]);
    println!("  {:34} {}", "Type", file_type(ehdr.e_type()));
    println!("  {:34} {}", "Machine", elf::machine_name(ehdr.e_machine()));
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

fn print_sections(reader: &elf::Reader, print_info: bool) {
    let sections = reader.section_headers();
    if print_info {
        println!("There are {} section headers, starting at offset 0x{:x}:",
                 sections.len(), reader.elf_header().e_shoff());
    }
    println!("\nSection Headers:");
    println!("  [Nr] {:17} {:15} {:8} {:6} {:6} ES Flg Lk Inf Al",
             "Name", "Type", "Addr", "Off", "Size");
    for (index, shdr) in sections.into_iter().enumerate() {
        let name = to_utf8(reader.section_name(shdr));
        let (flags, len) = section_flags(shdr.sh_flags());
        let flags = str::from_utf8(&flags[..len]).unwrap();
        println!("  [{:2}] {:17} {:15} {:08x} {:06x} {:06x} {:02x} {:>3} {:>2} {:>3} {:>2}",
                 index,
                 name,
                 elf::section_type_name(shdr.sh_type()),
                 shdr.sh_addr(),
                 shdr.sh_offset(),
                 shdr.sh_size(),
                 shdr.sh_entsize(),
                 &flags[..len],
                 shdr.sh_link(),
                 shdr.sh_info(),
                 shdr.sh_addralign());
    }
    println!("Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  p (processor specific)");
}

fn print_segments(reader: &elf::Reader, print_info: bool) {
    let segments = reader.program_headers();
    if segments.is_empty() {
        println!("\nThere are no program headers in this file.");
        return;
    }
    if print_info {
        let header = reader.elf_header();
        println!("\nElf file type is {}", file_type(header.e_type()));
        println!("Entry point 0x{:x}", header.e_entry());
        println!("There are {} program headers, starting at offset {}",
                 segments.len(), header.e_phoff());
    }

    println!("\nProgram Headers:");
    println!("  Type           Offset   VirtAddr   PhysAddr   FileSiz MemSiz  Flg Align");
    for phdr in segments {
        // 32-bit
        let flags = phdr.p_flags();
        println!("  {:14} 0x{:06x} 0x{:08x} 0x{:08x} 0x{:05x} 0x{:05x} {}{}{} 0x{:x}",
                 elf::segment_type_name(phdr.p_type()),
                 phdr.p_offset(),
                 phdr.p_vaddr(),
                 phdr.p_paddr(),
                 phdr.p_filesz(),
                 phdr.p_memsz(),
                 if flags & elf::PF_R != 0 { 'R' } else { ' ' },
                 if flags & elf::PF_W != 0 { 'W' } else { ' ' },
                 if flags & elf::PF_X != 0 { 'E' } else { ' ' },
                 phdr.p_align());
    }

    println!("\n Section to Segment mapping:");
    println!("  Segment Sections...");
    let sections = reader.section_headers();
    for (index, phdr) in segments.into_iter().enumerate() {
        print!("   {:02}    ", index);
        let p_vaddr = phdr.p_vaddr();
        let p_vaddr_end = p_vaddr + phdr.p_memsz();
        for shdr in sections.into_iter()
            .filter(|shdr| shdr.sh_flags() & elf::SHF_ALLOC != 0)
        {
            let sh_addr = shdr.sh_addr();
            let sh_addr_end = sh_addr + shdr.sh_size();
            if max(p_vaddr, sh_addr) < min(p_vaddr_end, sh_addr_end) {
                print!(" {}", to_utf8(reader.section_name(shdr)));
            }
        }
        println!();
    }
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

fn print_symbols(reader: &elf::Reader, dynamic: bool) -> Result<()> {
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
                 elf::symbol_type_name(symbol.symbol_type()), elf::symbol_binding_name(symbol.binding()),
                 elf::symbol_visibility_name(symbol.visibility()), symbol_index(symbol.section), name);
    }

    Ok(())
}

fn print_relocations(reader: &elf::Reader) -> Result<()> {
    let machine = reader.elf_header().e_machine();
    for section in reader.sections_matching(|shdr| { let t = shdr.sh_type(); t == elf::SHT_REL || t == elf::SHT_RELA}) {
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

fn hex_string(data: &[u8]) -> String {
    let mut s = String::with_capacity(2*data.len());
    for b in data {
        s += &format!("{:02x}", b);
    }
    s
}

fn print_notes(reader: &elf::Reader) -> Result<()> {
    for section in reader.sections_matching(|shdr| shdr.sh_type() == elf::SHT_NOTE) {
        if let elf::SectionDataRef::NoteTable(notes) = section.data {
            println!("\nDisplaying notes found in: {}", section.name.map_or("", to_utf8));
            println!("  {:14} Type Desc", "Name");
            for note in notes {
                let name = note.name.map_or("<none>", to_utf8);
                let desc = note.desc.map_or(String::from("<none>"),
                                            |s| str::from_utf8(s).map(String::from)
                                                .unwrap_or_else(|_| hex_string(s)));
                // Not the greatest implementation.
                println!("  {:14} {:4} {}", name, note.note_type, desc);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
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
        .arg(Arg::with_name("notes")
             .help("Display the core notes (if present)")
             .short("n")
             .long("notes"))
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
    let program_headers = headers || matches.is_present("program-headers") || matches.is_present("segments");
    let section_headers = headers || matches.is_present("section-headers") || matches.is_present("sections");
    let symbols = all || matches.is_present("syms") || matches.is_present("symbols");
    let dynsyms = matches.is_present("dyn-sym");
    let notes = matches.is_present("notes");
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
        if program_headers {
            print_segments(&reader, !file_header);
        }
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
        if notes {
            print_notes(&reader)?;
        }
        // TODO: histogram
    }
    Ok(())
}
