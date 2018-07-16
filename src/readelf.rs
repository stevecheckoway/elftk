extern crate elftk;
extern crate failure;

mod names;

use std::fs;
use std::io;
use std::str;

use elftk as elf;
use names::*;

fn to_utf8(s: &[u8]) -> &str {
    str::from_utf8(s).unwrap_or("<invalid>")
}

fn print_sections(reader: &elf::Reader) {
    let sections = reader.section_headers();
    let offset = reader.header().e_shoff();
    println!("There are {} section headers, starting at offset 0x{:x}:\n", sections.len(), offset);
    println!("Section Headers:");
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

fn print_symbols(reader: &elf::Reader) -> Result<(), Error> {
    let symtab_section = if let Some(s) = reader.symtab()? {
        s
    } else {
        return Ok(());
    };
    let symtab = if let elf::SectionDataRef::SymbolTable(s) = symtab_section.data { s } else { unreachable!() };
    let section_name = symtab_section.name.map_or("", to_utf8);
    println!("\nSymbol table '{}' contains {} entries:", section_name, symtab.len());
    println!("   Num:    Value  Size Type    Bind   Vis      Ndx Name");
    for i in 0 .. symtab.len() {
        let symbol = symtab.get(i)?;
        let name = symbol.symbol_name.map_or("", to_utf8);
        println!("{:6}: {:08x} {:5} {:<7} {:<6} {:<6} {:>3} {}",
                 i, symbol.value, symbol.size,
                 symbol_type(symbol.symbol_type()), symbol_binding(symbol.binding()),
                 symbol_visibility(symbol.visibility()), symbol_index(symbol.section), name);
    }

    Ok(())
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
    print_symbols(&reader)?;
    Ok(())
}
