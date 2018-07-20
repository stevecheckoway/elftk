#![allow(non_camel_case_types)]
#![allow(dead_code)]

use super::{Elf_Half, Elf_Word, Elf_Xword};

macro_rules! constants {
    ($name:ident, $t:ty, $($id:ident = $val:expr ; $display:expr),*) => {
        $( pub const $id: $t = $val; )*
        pub fn $name(v: $t) -> &'static str {
            match v {
                $( $id => $display, )*
                _ => ""
            }
        }
    }
}

// Elf header constants.
// e_ident
pub const EI_MAG0:       usize = 0;  // File identification
pub const EI_MAG1:       usize = 1;  // File identification
pub const EI_MAG2:       usize = 2;  // File identification
pub const EI_MAG3:       usize = 3;  // File identification
pub const EI_CLASS:      usize = 4;  // File class
pub const EI_DATA:       usize = 5;  // Data encoding
pub const EI_VERSION:    usize = 6;  // File version
pub const EI_OSABI:      usize = 7;  // Operating system/ABI identification
pub const EI_ABIVERSION: usize = 8;  // ABI version
pub const EI_PAD:        usize = 9;  // Start of padding bytes

// EI_MAG0 to EI_MAG3
pub const ELFMAG0: u8 = 0x7f; // e_ident[EI_MAG0]
pub const ELFMAG1: u8 = b'E'; // e_ident[EI_MAG1]
pub const ELFMAG2: u8 = b'L'; // e_ident[EI_MAG2]
pub const ELFMAG3: u8 = b'F'; // e_ident[EI_MAG3]

// EI_CLASS
constants!(class_name, u8,
           ELFCLASSNONE = 0; "NONE",   // Invalid class
           ELFCLASS32   = 1; "ELF32",  // 32-bit objects
           ELFCLASS64   = 2; "ELF64"); // 64-bit objects

// EI_DATA
pub const ELFDATANONE: u8 = 0; // Invalid data encoding
pub const ELFDATA2LSB: u8 = 1; // Little endian
pub const ELFDATA2MSB: u8 = 2; // Big endian

// EI_VERSION must be EV_CURRENT

// EI_OSABI
constants!(osabi_name, u8,
           ELFOSABI_NONE     = 0;  "UNIX - System V",
           ELFOSABI_HPUX     = 1;  "Hewlett-Packard HP-UX",
           ELFOSABI_NETBSD   = 2;  "NetBSD",
           ELFOSABI_GNU      = 3;  "GNU",
           ELFOSABI_SOLARIS  = 6;  "Sun Solaris",
           ELFOSABI_AIX      = 7;  "AIX",
           ELFOSABI_IRIX     = 8;  "IRIX",
           ELFOSABI_FREEBSD  = 9;  "FreeBSD",
           ELFOSABI_TRU64    = 10; "Compaq TRU64 UNIX",
           ELFOSABI_MODESTO  = 11; "Novell Modesto",
           ELFOSABI_OPENBSD  = 12; "Open BSD",
           ELFOSABI_OPENVMS  = 13; "Open VMS",
           ELFOSABI_NSK      = 14; "Hewlett-Packard Non-Stop Kernel",
           ELFOSABI_AROS     = 15; "Amiga Research OS",
           ELFOSABI_FENIXOS  = 16; "The FenixOS highly scalable multi-core OS",
           ELFOSABI_CLOUDABI = 17; "Nuxi CloudABI",
           ELFOSABI_OPENVOS  = 18; "Stratus Technologies OpenVOS");
pub const ELFOSABI_LINUX:    u8 = 3;  // Linux historical - alias for ELFOSABI_GNU

// EI_ABIVERSION should likely be 0

// EI_PAD through EI_NIDENT should be 0

// e_type
constants!(type_name, Elf_Half,
           ET_NONE = 0; "NONE",
           ET_REL  = 1; "REL",
           ET_EXEC = 2; "EXEC",
           ET_DYN  = 3; "DYN",
           ET_CORE = 4; "CORE");
pub const ET_LOOS:   Elf_Half = 0xfe00; // Operating system-specific
pub const ET_HIOS:   Elf_Half = 0xfeff; // Operating system-specific
pub const ET_LOPROC: Elf_Half = 0xff00; // Processor-specific
pub const ET_HIPROC: Elf_Half = 0xffff; // Processor-specific

// e_machine
constants!(machine_name, Elf_Half,
           EM_NONE          = 0;   "No machine",
           EM_M32           = 1;   "AT&T WE 32100",
           EM_SPARC         = 2;   "SPARC",
           EM_386           = 3;   "Intel 80386",
           EM_68K           = 4;   "Motorola 68000",
           EM_88K           = 5;   "Motorola 88000",
           EM_IAMCU         = 6;   "Intel MCU",
           EM_860           = 7;   "Intel 80860",
           EM_MIPS          = 8;   "MIPS I Architecture",
           EM_S370          = 9;   "IBM System/370 Processor",
           EM_MIPS_RS3_LE   = 10;  "MIPS RS3000 Little-endian",
           EM_PARISC        = 15;  "Hewlett-Packard PA-RISC",
           EM_VPP500        = 17;  "Fujitsu VPP500",
           EM_SPARC32PLUS   = 18;  "Enhanced instruction set SPARC",
           EM_960           = 19;  "Intel 80960",
           EM_PPC           = 20;  "PowerPC",
           EM_PPC64         = 21;  "64-bit PowerPC",
           EM_S390          = 22;  "IBM System/390 Processor",
           EM_SPU           = 23;  "IBM SPU/SPC",
           EM_V800          = 36;  "NEC V800",
           EM_FR20          = 37;  "Fujitsu FR20",
           EM_RH32          = 38;  "TRW RH-32",
           EM_RCE           = 39;  "Motorola RCE",
           EM_ARM           = 40;  "ARM 32-bit architecture (AARCH32)",
           EM_ALPHA         = 41;  "Digital Alpha",
           EM_SH            = 42;  "Hitachi SH",
           EM_SPARCV9       = 43;  "SPARC Version 9",
           EM_TRICORE       = 44;  "Siemens TriCore embedded processor",
           EM_ARC           = 45;  "Argonaut RISC Core, Argonaut Technologies Inc.",
           EM_H8_300        = 46;  "Hitachi H8/300",
           EM_H8_300H       = 47;  "Hitachi H8/300H",
           EM_H8S           = 48;  "Hitachi H8S",
           EM_H8_500        = 49;  "Hitachi H8/500",
           EM_IA_64         = 50;  "Intel IA-64 processor architecture",
           EM_MIPS_X        = 51;  "Stanford MIPS-X",
           EM_COLDFIRE      = 52;  "Motorola ColdFire",
           EM_68HC12        = 53;  "Motorola M68HC12",
           EM_MMA           = 54;  "Fujitsu MMA Multimedia Accelerator",
           EM_PCP           = 55;  "Siemens PCP",
           EM_NCPU          = 56;  "Sony nCPU embedded RISC processor",
           EM_NDR1          = 57;  "Denso NDR1 microprocessor",
           EM_STARCORE      = 58;  "Motorola Star*Core processor",
           EM_ME16          = 59;  "Toyota ME16 processor",
           EM_ST100         = 60;  "STMicroelectronics ST100 processor",
           EM_TINYJ         = 61;  "Advanced Logic Corp. TinyJ embedded processor family",
           EM_X86_64        = 62;  "Advanced Micro Devices X86-64",
           EM_PDSP          = 63;  "Sony DSP Processor",
           EM_PDP10         = 64;  "Digital Equipment Corp. PDP-10",
           EM_PDP11         = 65;  "Digital Equipment Corp. PDP-11",
           EM_FX66          = 66;  "Siemens FX66 microcontroller",
           EM_ST9PLUS       = 67;  "STMicroelectronics ST9+ 8/16 bit microcontroller",
           EM_ST7           = 68;  "STMicroelectronics ST7 8-bit microcontroller",
           EM_68HC16        = 69;  "Motorola MC68HC16 Microcontroller",
           EM_68HC11        = 70;  "Motorola MC68HC11 Microcontroller",
           EM_68HC08        = 71;  "Motorola MC68HC08 Microcontroller",
           EM_68HC05        = 72;  "Motorola MC68HC05 Microcontroller",
           EM_SVX           = 73;  "Silicon Graphics SVx",
           EM_ST19          = 74;  "STMicroelectronics ST19 8-bit microcontroller",
           EM_VAX           = 75;  "Digital VAX",
           EM_CRIS          = 76;  "Axis Communications 32-bit embedded processor",
           EM_JAVELIN       = 77;  "Infineon Technologies 32-bit embedded processor",
           EM_FIREPATH      = 78;  "Element 14 64-bit DSP Processor",
           EM_ZSP           = 79;  "LSI Logic 16-bit DSP Processor",
           EM_MMIX          = 80;  "Donald Knuth's educational 64-bit processor",
           EM_HUANY         = 81;  "Harvard University machine-independent object files",
           EM_PRISM         = 82;  "SiTera Prism",
           EM_AVR           = 83;  "Atmel AVR 8-bit microcontroller",
           EM_FR30          = 84;  "Fujitsu FR30",
           EM_D10V          = 85;  "Mitsubishi D10V",
           EM_D30V          = 86;  "Mitsubishi D30V",
           EM_V850          = 87;  "NEC v850",
           EM_M32R          = 88;  "Mitsubishi M32R",
           EM_MN10300       = 89;  "Matsushita MN10300",
           EM_MN10200       = 90;  "Matsushita MN10200",
           EM_PJ            = 91;  "picoJava",
           EM_OPENRISC      = 92;  "OpenRISC 32-bit embedded processor",
           EM_ARC_COMPACT   = 93;  "ARC International ARCompact processor (old spelling/synonym: EM_ARC_A5)",
           EM_XTENSA        = 94;  "Tensilica Xtensa Architecture",
           EM_VIDEOCORE     = 95;  "Alphamosaic VideoCore processor",
           EM_TMM_GPP       = 96;  "Thompson Multimedia General Purpose Processor",
           EM_NS32K         = 97;  "National Semiconductor 32000 series",
           EM_TPC           = 98;  "Tenor Network TPC processor",
           EM_SNP1K         = 99;  "Trebia SNP 1000 processor",
           EM_ST200         = 100; "STMicroelectronics (www.st.com) ST200 microcontroller",
           EM_IP2K          = 101; "Ubicom IP2xxx microcontroller family",
           EM_MAX           = 102; "MAX Processor",
           EM_CR            = 103; "National Semiconductor CompactRISC microprocessor",
           EM_F2MC16        = 104; "Fujitsu F2MC16",
           EM_MSP430        = 105; "Texas Instruments embedded microcontroller msp430",
           EM_BLACKFIN      = 106; "Analog Devices Blackfin (DSP) processor",
           EM_SE_C33        = 107; "S1C33 Family of Seiko Epson processors",
           EM_SEP           = 108; "Sharp embedded microprocessor",
           EM_ARCA          = 109; "Arca RISC Microprocessor",
           EM_UNICORE       = 110; "Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University",
           EM_EXCESS        = 111; "eXcess: 16/32/64-bit configurable embedded CPU",
           EM_DXP           = 112; "Icera Semiconductor Inc. Deep Execution Processor",
           EM_ALTERA_NIOS2  = 113; "Altera Nios II soft-core processor",
           EM_CRX           = 114; "National Semiconductor CompactRISC CRX microprocessor",
           EM_XGATE         = 115; "Motorola XGATE embedded processor",
           EM_C166          = 116; "Infineon C16x/XC16x processor",
           EM_M16C          = 117; "Renesas M16C series microprocessors",
           EM_DSPIC30F      = 118; "Microchip Technology dsPIC30F Digital Signal Controller",
           EM_CE            = 119; "Freescale Communication Engine RISC core",
           EM_M32C          = 120; "Renesas M32C series microprocessors",
           EM_TSK3000       = 131; "Altium TSK3000 core",
           EM_RS08          = 132; "Freescale RS08 embedded processor",
           EM_SHARC         = 133; "Analog Devices SHARC family of 32-bit DSP processors",
           EM_ECOG2         = 134; "Cyan Technology eCOG2 microprocessor",
           EM_SCORE7        = 135; "Sunplus S+core7 RISC processor",
           EM_DSP24         = 136; "New Japan Radio (NJR) 24-bit DSP Processor",
           EM_VIDEOCORE3    = 137; "Broadcom VideoCore III processor",
           EM_LATTICEMICO32 = 138; "RISC processor for Lattice FPGA architecture",
           EM_SE_C17        = 139; "Seiko Epson C17 family",
           EM_TI_C6000      = 140; "The Texas Instruments TMS320C6000 DSP family",
           EM_TI_C2000      = 141; "The Texas Instruments TMS320C2000 DSP family",
           EM_TI_C5500      = 142; "The Texas Instruments TMS320C55x DSP family",
           EM_TI_ARP32      = 143; "Texas Instruments Application Specific RISC Processor, 32bit fetch",
           EM_TI_PRU        = 144; "Texas Instruments Programmable Realtime Unit",
           EM_MMDSP_PLUS    = 160; "STMicroelectronics 64bit VLIW Data Signal Processor",
           EM_CYPRESS_M8C   = 161; "Cypress M8C microprocessor",
           EM_R32C          = 162; "Renesas R32C series microprocessors",
           EM_TRIMEDIA      = 163; "NXP Semiconductors TriMedia architecture family",
           EM_QDSP6         = 164; "QUALCOMM DSP6 Processor",
           EM_8051          = 165; "Intel 8051 and variants",
           EM_STXP7X        = 166; "STMicroelectronics STxP7x family of configurable and extensible RISC processors",
           EM_NDS32         = 167; "Andes Technology compact code size embedded RISC processor family",
           EM_ECOG1         = 168; "Cyan Technology eCOG1X family",
           EM_MAXQ30        = 169; "Dallas Semiconductor MAXQ30 Core Micro-controllers",
           EM_XIMO16        = 170; "New Japan Radio (NJR) 16-bit DSP Processor",
           EM_MANIK         = 171; "M2000 Reconfigurable RISC Microprocessor",
           EM_CRAYNV2       = 172; "Cray Inc. NV2 vector architecture",
           EM_RX            = 173; "Renesas RX family",
           EM_METAG         = 174; "Imagination Technologies META processor architecture",
           EM_MCST_ELBRUS   = 175; "MCST Elbrus general purpose hardware architecture",
           EM_ECOG16        = 176; "Cyan Technology eCOG16 family",
           EM_CR16          = 177; "National Semiconductor CompactRISC CR16 16-bit microprocessor",
           EM_ETPU          = 178; "Freescale Extended Time Processing Unit",
           EM_SLE9X         = 179; "Infineon Technologies SLE9X core",
           EM_L10M          = 180; "Intel L10M",
           EM_K10M          = 181; "Intel K10M",
           EM_AARCH64       = 183; "ARM 64-bit architecture (AARCH64)",
           EM_AVR32         = 185; "Atmel Corporation 32-bit microprocessor family",
           EM_STM8          = 186; "STMicroeletronics STM8 8-bit microcontroller",
           EM_TILE64        = 187; "Tilera TILE64 multicore architecture family",
           EM_TILEPRO       = 188; "Tilera TILEPro multicore architecture family",
           EM_MICROBLAZE    = 189; "Xilinx MicroBlaze 32-bit RISC soft processor core",
           EM_CUDA          = 190; "NVIDIA CUDA architecture",
           EM_TILEGX        = 191; "Tilera TILE-Gx multicore architecture family",
           EM_CLOUDSHIELD   = 192; "CloudShield architecture family",
           EM_COREA_1ST     = 193; "KIPO-KAIST Core-A 1st generation processor family",
           EM_COREA_2ND     = 194; "KIPO-KAIST Core-A 2nd generation processor family",
           EM_ARC_COMPACT2  = 195; "Synopsys ARCompact V2",
           EM_OPEN8         = 196; "Open8 8-bit RISC soft processor core",
           EM_RL78          = 197; "Renesas RL78 family",
           EM_VIDEOCORE5    = 198; "Broadcom VideoCore V processor",
           EM_78KOR         = 199; "Renesas 78KOR family",
           EM_56800EX       = 200; "Freescale 56800EX Digital Signal Controller (DSC)",
           EM_BA1           = 201; "Beyond BA1 CPU architecture",
           EM_BA2           = 202; "Beyond BA2 CPU architecture",
           EM_XCORE         = 203; "XMOS xCORE processor family",
           EM_MCHP_PIC      = 204; "Microchip 8-bit PIC(r) family",
           EM_INTEL205      = 205; "Reserved by Intel",
           EM_INTEL206      = 206; "Reserved by Intel",
           EM_INTEL207      = 207; "Reserved by Intel",
           EM_INTEL208      = 208; "Reserved by Intel",
           EM_INTEL209      = 209; "Reserved by Intel",
           EM_KM32          = 210; "KM211 KM32 32-bit processor",
           EM_KMX32         = 211; "KM211 KMX32 32-bit processor",
           EM_KMX16         = 212; "KM211 KMX16 16-bit processor",
           EM_KMX8          = 213; "KM211 KMX8 8-bit processor",
           EM_KVARC         = 214; "KM211 KVARC processor",
           EM_CDP           = 215; "Paneve CDP architecture family",
           EM_COGE          = 216; "Cognitive Smart Memory Processor",
           EM_COOL          = 217; "Bluechip Systems CoolEngine",
           EM_NORC          = 218; "Nanoradio Optimized RISC",
           EM_CSR_KALIMBA   = 219; "CSR Kalimba architecture family",
           EM_Z80           = 220; "Zilog Z80",
           EM_VISIUM        = 221; "Controls and Data Services VISIUMcore processor",
           EM_FT32          = 222; "FTDI Chip FT32 high performance 32-bit RISC architecture",
           EM_MOXIE         = 223; "Moxie processor family",
           EM_AMDGPU        = 224; "AMD GPU architecture",
           EM_RISCV         = 243; "RISC-V");
pub const EM_ECOG1X: Elf_Half = EM_ECOG1; // Cyan Technology eCOG1X family

// e_version
pub const EV_NONE:    Elf_Word = 0; // Invalid version
pub const EV_CURRENT: Elf_Word = 1; // Current version


// Special section indexes
pub const SHN_UNDEF:     Elf_Half = 0;
pub const SHN_LORESERVE: Elf_Half = 0xff00;
pub const SHN_LOPROC:    Elf_Half = 0xff00;
pub const SHN_HIPROC:    Elf_Half = 0xff1f;
pub const SHN_LOOS:      Elf_Half = 0xff20;
pub const SHN_HIOS:      Elf_Half = 0xff3f;
pub const SHN_ABS:       Elf_Half = 0xfff1;
pub const SHN_COMMON:    Elf_Half = 0xfff2;
pub const SHN_XINDEX:    Elf_Half = 0xffff;
pub const SHN_HIRESERVE: Elf_Half = 0xffff;

// Section Types sh_type
constants!(section_type_name, Elf_Word,
           SHT_NULL          = 0;  "NULL",
           SHT_PROGBITS      = 1;  "PROGBITS",
           SHT_SYMTAB        = 2;  "SYMTAB",
           SHT_STRTAB        = 3;  "STRTAB",
           SHT_RELA          = 4;  "RELA",
           SHT_HASH          = 5;  "HASH",
           SHT_DYNAMIC       = 6;  "DYNAMIC",
           SHT_NOTE          = 7;  "NOTE",
           SHT_NOBITS        = 8;  "NOBITS",
           SHT_REL           = 9;  "REL",
           SHT_SHLIB         = 10; "SHLIB",
           SHT_DYNSYM        = 11; "DYNSYM",
           SHT_INIT_ARRAY    = 14; "INIT_ARRAY",
           SHT_FINI_ARRAY    = 15; "FINI_ARRAY",
           SHT_PREINIT_ARRAY = 16; "PREINIT_ARRAY",
           SHT_GROUP         = 17; "GROUP",
           SHT_SYMTAB_SHNDX  = 18; "SYMTAB_SHNDX");
pub const SHT_LOOS:          Elf_Word = 0x6000_0000;
pub const SHT_HIOS:          Elf_Word = 0x6fff_ffff;
pub const SHT_LOPROC:        Elf_Word = 0x7000_0000;
pub const SHT_HIPROC:        Elf_Word = 0x7fff_ffff;
pub const SHT_LOUSER:        Elf_Word = 0x8000_0000;
pub const SHT_HIUSER:        Elf_Word = 0xffff_ffff;

// sh_flags
pub const SHF_WRITE:            Elf_Xword = 0x1;
pub const SHF_ALLOC:            Elf_Xword = 0x2;
pub const SHF_EXECINSTR:        Elf_Xword = 0x4;
pub const SHF_MERGE:            Elf_Xword = 0x10;
pub const SHF_STRINGS:          Elf_Xword = 0x20;
pub const SHF_INFO_LINK:        Elf_Xword = 0x40;
pub const SHF_LINK_ORDER:       Elf_Xword = 0x80;
pub const SHF_OS_NONCONFORMING: Elf_Xword = 0x100;
pub const SHF_GROUP:            Elf_Xword = 0x200;
pub const SHF_TLS:              Elf_Xword = 0x400;
pub const SHF_COMPRESSED:       Elf_Xword = 0x800;
pub const SHF_MASKOS:           Elf_Xword = 0x0ff0_0000;
pub const SHF_MASKPROC:         Elf_Xword = 0xf000_0000;
pub const SHF_EXCLUDED:         Elf_Xword = 0x8000_0000;

// ch_type
pub const ELFCOMPRESS_ZLIB:   Elf_Word = 1;
pub const ELFCOMPRESS_LOOS:   Elf_Word = 0x6000_0000;
pub const ELFCOMPRESS_HIOS:   Elf_Word = 0x6fff_ffff;
pub const ELFCOMPRESS_LOPROC: Elf_Word = 0x7000_0000;
pub const ELFCOMPRESS_HIPROC: Elf_Word = 0x7fff_ffff;

// Section group flags
pub const GRP_COMDAT:   Elf_Word = 0x1;
pub const GRP_MASKOS:   Elf_Word = 0x0ff0_0000;
pub const GRP_MASKPROC: Elf_Word = 0xf000_0000;


// Symbol table
pub const STN_UNDEF: Elf_Half = 0;

// st_info
constants!(symbol_binding_name, u8,
           STB_LOCAL  = 0; "LOCAL",
           STB_GLOBAL = 1; "GLOBAL",
           STB_WEAK   = 2; "WEAK");
pub const STB_LOOS:   u8 = 10;
pub const STB_HIOS:   u8 = 12;
pub const STB_LOPROC: u8 = 13;
pub const STB_HIPROC: u8 = 15;

constants!(symbol_type_name, u8,
           STT_NOTYPE  = 0; "NOTYPE",
           STT_OBJECT  = 1; "OBJECT",
           STT_FUNC    = 2; "FUNC",
           STT_SECTION = 3; "SECTION",
           STT_FILE    = 4; "FILE",
           STT_COMMON  = 5; "COMMON",
           STT_TLS     = 6; "TLS");
pub const STT_LOOS:    u8 = 10;
pub const STT_HIOS:    u8 = 12;
pub const STT_LOPROC:  u8 = 13;
pub const STT_HIPROC:  u8 = 15;

// st_other
constants!(symbol_visibility_name, u8,
           STV_DEFAULT   = 0; "DEFAULT",
           STV_INTERNAL  = 1; "INTERNAL",
           STV_HIDDEN    = 2; "HIDDEN",
           STV_PROTECTED = 3; "PROTECTED");

// Segments
// p_type
constants!(segment_type_name, Elf_Word,
           PT_NULL    = 0; "NULL",
           PT_LOAD    = 1; "LOAD",
           PT_DYNAMIC = 2; "DYNAMIC",
           PT_INTERP  = 3; "INTERP",
           PT_NOTE    = 4; "NOTE",
           PT_SHLIB   = 5; "SHLIB",
           PT_PHDR    = 6; "PHDR",
           PT_TLS     = 7; "TLS");
pub const PT_LOOS:   Elf_Word = 0x6000_0000;
pub const PT_HIOS:   Elf_Word = 0x6fff_ffff;
pub const PT_LOPROC: Elf_Word = 0x7000_0000;
pub const PT_HIPROC: Elf_Word = 0x7fff_ffff;

// p_flags
pub const PF_X:        Elf_Word = 1;
pub const PF_W:        Elf_Word = 2;
pub const PF_R:        Elf_Word = 4;
pub const PF_MASKOS:   Elf_Word = 0x0ff0_0000;
pub const PF_MASKPROC: Elf_Word = 0xf000_0000;
