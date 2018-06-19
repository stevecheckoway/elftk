#![allow(non_camel_case_types)]
#![allow(dead_code)]

use elf_base_types::{Elf_Half, Elf_Word};

// Elf header constants.
// e_ident
pub const EI_MAG0:       usize = 0;  /// File identification
pub const EI_MAG1:       usize = 1;  /// File identification
pub const EI_MAG2:       usize = 2;  /// File identification
pub const EI_MAG3:       usize = 3;  /// File identification
pub const EI_CLASS:      usize = 4;  /// File class
pub const EI_DATA:       usize = 5;  /// Data encoding
pub const EI_VERSION:    usize = 6;  /// File version
pub const EI_OSABI:      usize = 7;  /// Operating system/ABI identification
pub const EI_ABIVERSION: usize = 8;  /// ABI version
pub const EI_PAD:        usize = 9;  /// Start of padding bytes

// EI_MAG0 to EI_MAG3
pub const ELFMAG0: u8 = 0x7f; /// e_ident[EI_MAG0]
pub const ELFMAG1: u8 = b'E';  /// e_ident[EI_MAG1]
pub const ELFMAG2: u8 = b'L';  /// e_ident[EI_MAG2]
pub const ELFMAG3: u8 = b'F';  /// e_ident[EI_MAG3]

// EI_CLASS
pub const ELFCLASSNONE: u8 = 0; /// Invalid class
pub const ELFCLASS32:   u8 = 1; /// 32-bit objects
pub const ELFCLASS64:   u8 = 2; /// 64-bit objects

// EI_DATA
pub const ELFDATANONE: u8 = 0; /// Invalid data encoding
pub const ELFDATA2LSB: u8 = 1; /// Little endian
pub const ELFDATA2MSB: u8 = 2; /// Big endian

// EI_VERSION must be EV_CURRENT

// EI_OSABI
pub const ELFOSABI_NONE:     u8 = 0;  /// No extensions or unspecified
pub const ELFOSABI_HPUX:     u8 = 1;  /// Hewlett-Packard HP-UX
pub const ELFOSABI_NETBSD:   u8 = 2;  /// NetBSD
pub const ELFOSABI_GNU:      u8 = 3;  /// GNU
pub const ELFOSABI_LINUX:    u8 = 3;  /// Linux historical - alias for ELFOSABI_GNU
pub const ELFOSABI_SOLARIS:  u8 = 6;  /// Sun Solaris
pub const ELFOSABI_AIX:      u8 = 7;  /// AIX
pub const ELFOSABI_IRIX:     u8 = 8;  /// IRIX
pub const ELFOSABI_FREEBSD:  u8 = 9;  /// FreeBSD
pub const ELFOSABI_TRU64:    u8 = 10; /// Compaq TRU64 UNIX
pub const ELFOSABI_MODESTO:  u8 = 11; /// Novell Modesto
pub const ELFOSABI_OPENBSD:  u8 = 12; /// Open BSD
pub const ELFOSABI_OPENVMS:  u8 = 13; /// Open VMS
pub const ELFOSABI_NSK:      u8 = 14; /// Hewlett-Packard Non-Stop Kernel
pub const ELFOSABI_AROS:     u8 = 15; /// Amiga Research OS
pub const ELFOSABI_FENIXOS:  u8 = 16; /// The FenixOS highly scalable multi-core OS
pub const ELFOSABI_CLOUDABI: u8 = 17; /// Nuxi CloudABI
pub const ELFOSABI_OPENVOS:  u8 = 18; /// Stratus Technologies OpenVOS

// EI_ABIVERSION should likely be 0

// EI_PAD through EI_NIDENT should be 0

// e_type
pub const ET_NONE:   Elf_Half = 0;      /// No file type
pub const ET_REL:    Elf_Half = 1;      /// Relocatable file
pub const ET_EXEC:   Elf_Half = 2;      /// Executable file
pub const ET_DYN:    Elf_Half = 3;      /// Shared object file
pub const ET_CORE:   Elf_Half = 4;      /// Core file
pub const ET_LOOS:   Elf_Half = 0xfe00; /// Operating system-specific
pub const ET_HIOS:   Elf_Half = 0xfeff; /// Operating system-specific
pub const ET_LOPROC: Elf_Half = 0xff00; /// Processor-specific
pub const ET_HIPROC: Elf_Half = 0xffff; /// Processor-specific

// e_machine
pub const EM_NONE:          Elf_Half = 0;   /// No machine
pub const EM_M32:           Elf_Half = 1;   /// AT&T WE 32100
pub const EM_SPARC:         Elf_Half = 2;   /// SPARC
pub const EM_386:           Elf_Half = 3;   /// Intel 80386
pub const EM_68K:           Elf_Half = 4;   /// Motorola 68000
pub const EM_88K:           Elf_Half = 5;   /// Motorola 88000
pub const EM_IAMCU:         Elf_Half = 6;   /// Intel MCU
pub const EM_860:           Elf_Half = 7;   /// Intel 80860
pub const EM_MIPS:          Elf_Half = 8;   /// MIPS I Architecture
pub const EM_S370:          Elf_Half = 9;   /// IBM System/370 Processor
pub const EM_MIPS_RS3_LE:   Elf_Half = 10;  /// MIPS RS3000 Little-endian
pub const EM_PARISC:        Elf_Half = 15;  /// Hewlett-Packard PA-RISC
pub const EM_VPP500:        Elf_Half = 17;  /// Fujitsu VPP500
pub const EM_SPARC32PLUS:   Elf_Half = 18;  /// Enhanced instruction set SPARC
pub const EM_960:           Elf_Half = 19;  /// Intel 80960
pub const EM_PPC:           Elf_Half = 20;  /// PowerPC
pub const EM_PPC64:         Elf_Half = 21;  /// 64-bit PowerPC
pub const EM_S390:          Elf_Half = 22;  /// IBM System/390 Processor
pub const EM_SPU:           Elf_Half = 23;  /// IBM SPU/SPC
pub const EM_V800:          Elf_Half = 36;  /// NEC V800
pub const EM_FR20:          Elf_Half = 37;  /// Fujitsu FR20
pub const EM_RH32:          Elf_Half = 38;  /// TRW RH-32
pub const EM_RCE:           Elf_Half = 39;  /// Motorola RCE
pub const EM_ARM:           Elf_Half = 40;  /// ARM 32-bit architecture (AARCH32)
pub const EM_ALPHA:         Elf_Half = 41;  /// Digital Alpha
pub const EM_SH:            Elf_Half = 42;  /// Hitachi SH
pub const EM_SPARCV9:       Elf_Half = 43;  /// SPARC Version 9
pub const EM_TRICORE:       Elf_Half = 44;  /// Siemens TriCore embedded processor
pub const EM_ARC:           Elf_Half = 45;  /// Argonaut RISC Core, Argonaut Technologies Inc.
pub const EM_H8_300:        Elf_Half = 46;  /// Hitachi H8/300
pub const EM_H8_300H:       Elf_Half = 47;  /// Hitachi H8/300H
pub const EM_H8S:           Elf_Half = 48;  /// Hitachi H8S
pub const EM_H8_500:        Elf_Half = 49;  /// Hitachi H8/500
pub const EM_IA_64:         Elf_Half = 50;  /// Intel IA-64 processor architecture
pub const EM_MIPS_X:        Elf_Half = 51;  /// Stanford MIPS-X
pub const EM_COLDFIRE:      Elf_Half = 52;  /// Motorola ColdFire
pub const EM_68HC12:        Elf_Half = 53;  /// Motorola M68HC12
pub const EM_MMA:           Elf_Half = 54;  /// Fujitsu MMA Multimedia Accelerator
pub const EM_PCP:           Elf_Half = 55;  /// Siemens PCP
pub const EM_NCPU:          Elf_Half = 56;  /// Sony nCPU embedded RISC processor
pub const EM_NDR1:          Elf_Half = 57;  /// Denso NDR1 microprocessor
pub const EM_STARCORE:      Elf_Half = 58;  /// Motorola Star*Core processor
pub const EM_ME16:          Elf_Half = 59;  /// Toyota ME16 processor
pub const EM_ST100:         Elf_Half = 60;  /// STMicroelectronics ST100 processor
pub const EM_TINYJ:         Elf_Half = 61;  /// Advanced Logic Corp. TinyJ embedded processor family
pub const EM_X86_64:        Elf_Half = 62;  /// AMD x86-64 architecture
pub const EM_PDSP:          Elf_Half = 63;  /// Sony DSP Processor
pub const EM_PDP10:         Elf_Half = 64;  /// Digital Equipment Corp. PDP-10
pub const EM_PDP11:         Elf_Half = 65;  /// Digital Equipment Corp. PDP-11
pub const EM_FX66:          Elf_Half = 66;  /// Siemens FX66 microcontroller
pub const EM_ST9PLUS:       Elf_Half = 67;  /// STMicroelectronics ST9+ 8/16 bit microcontroller
pub const EM_ST7:           Elf_Half = 68;  /// STMicroelectronics ST7 8-bit microcontroller
pub const EM_68HC16:        Elf_Half = 69;  /// Motorola MC68HC16 Microcontroller
pub const EM_68HC11:        Elf_Half = 70;  /// Motorola MC68HC11 Microcontroller
pub const EM_68HC08:        Elf_Half = 71;  /// Motorola MC68HC08 Microcontroller
pub const EM_68HC05:        Elf_Half = 72;  /// Motorola MC68HC05 Microcontroller
pub const EM_SVX:           Elf_Half = 73;  /// Silicon Graphics SVx
pub const EM_ST19:          Elf_Half = 74;  /// STMicroelectronics ST19 8-bit microcontroller
pub const EM_VAX:           Elf_Half = 75;  /// Digital VAX
pub const EM_CRIS:          Elf_Half = 76;  /// Axis Communications 32-bit embedded processor
pub const EM_JAVELIN:       Elf_Half = 77;  /// Infineon Technologies 32-bit embedded processor
pub const EM_FIREPATH:      Elf_Half = 78;  /// Element 14 64-bit DSP Processor
pub const EM_ZSP:           Elf_Half = 79;  /// LSI Logic 16-bit DSP Processor
pub const EM_MMIX:          Elf_Half = 80;  /// Donald Knuth's educational 64-bit processor
pub const EM_HUANY:         Elf_Half = 81;  /// Harvard University machine-independent object files
pub const EM_PRISM:         Elf_Half = 82;  /// SiTera Prism
pub const EM_AVR:           Elf_Half = 83;  /// Atmel AVR 8-bit microcontroller
pub const EM_FR30:          Elf_Half = 84;  /// Fujitsu FR30
pub const EM_D10V:          Elf_Half = 85;  /// Mitsubishi D10V
pub const EM_D30V:          Elf_Half = 86;  /// Mitsubishi D30V
pub const EM_V850:          Elf_Half = 87;  /// NEC v850
pub const EM_M32R:          Elf_Half = 88;  /// Mitsubishi M32R
pub const EM_MN10300:       Elf_Half = 89;  /// Matsushita MN10300
pub const EM_MN10200:       Elf_Half = 90;  /// Matsushita MN10200
pub const EM_PJ:            Elf_Half = 91;  /// picoJava
pub const EM_OPENRISC:      Elf_Half = 92;  /// OpenRISC 32-bit embedded processor
pub const EM_ARC_COMPACT:   Elf_Half = 93;  /// ARC International ARCompact processor (old spelling/synonym: EM_ARC_A5)
pub const EM_XTENSA:        Elf_Half = 94;  /// Tensilica Xtensa Architecture
pub const EM_VIDEOCORE:     Elf_Half = 95;  /// Alphamosaic VideoCore processor
pub const EM_TMM_GPP:       Elf_Half = 96;  /// Thompson Multimedia General Purpose Processor
pub const EM_NS32K:         Elf_Half = 97;  /// National Semiconductor 32000 series
pub const EM_TPC:           Elf_Half = 98;  /// Tenor Network TPC processor
pub const EM_SNP1K:         Elf_Half = 99;  /// Trebia SNP 1000 processor
pub const EM_ST200:         Elf_Half = 100; /// STMicroelectronics (www.st.com) ST200 microcontroller
pub const EM_IP2K:          Elf_Half = 101; /// Ubicom IP2xxx microcontroller family
pub const EM_MAX:           Elf_Half = 102; /// MAX Processor
pub const EM_CR:            Elf_Half = 103; /// National Semiconductor CompactRISC microprocessor
pub const EM_F2MC16:        Elf_Half = 104; /// Fujitsu F2MC16
pub const EM_MSP430:        Elf_Half = 105; /// Texas Instruments embedded microcontroller msp430
pub const EM_BLACKFIN:      Elf_Half = 106; /// Analog Devices Blackfin (DSP) processor
pub const EM_SE_C33:        Elf_Half = 107; /// S1C33 Family of Seiko Epson processors
pub const EM_SEP:           Elf_Half = 108; /// Sharp embedded microprocessor
pub const EM_ARCA:          Elf_Half = 109; /// Arca RISC Microprocessor
pub const EM_UNICORE:       Elf_Half = 110; /// Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University
pub const EM_EXCESS:        Elf_Half = 111; /// eXcess: 16/32/64-bit configurable embedded CPU
pub const EM_DXP:           Elf_Half = 112; /// Icera Semiconductor Inc. Deep Execution Processor
pub const EM_ALTERA_NIOS2:  Elf_Half = 113; /// Altera Nios II soft-core processor
pub const EM_CRX:           Elf_Half = 114; /// National Semiconductor CompactRISC CRX microprocessor
pub const EM_XGATE:         Elf_Half = 115; /// Motorola XGATE embedded processor
pub const EM_C166:          Elf_Half = 116; /// Infineon C16x/XC16x processor
pub const EM_M16C:          Elf_Half = 117; /// Renesas M16C series microprocessors
pub const EM_DSPIC30F:      Elf_Half = 118; /// Microchip Technology dsPIC30F Digital Signal Controller
pub const EM_CE:            Elf_Half = 119; /// Freescale Communication Engine RISC core
pub const EM_M32C:          Elf_Half = 120; /// Renesas M32C series microprocessors
pub const EM_TSK3000:       Elf_Half = 131; /// Altium TSK3000 core
pub const EM_RS08:          Elf_Half = 132; /// Freescale RS08 embedded processor
pub const EM_SHARC:         Elf_Half = 133; /// Analog Devices SHARC family of 32-bit DSP processors
pub const EM_ECOG2:         Elf_Half = 134; /// Cyan Technology eCOG2 microprocessor
pub const EM_SCORE7:        Elf_Half = 135; /// Sunplus S+core7 RISC processor
pub const EM_DSP24:         Elf_Half = 136; /// New Japan Radio (NJR) 24-bit DSP Processor
pub const EM_VIDEOCORE3:    Elf_Half = 137; /// Broadcom VideoCore III processor
pub const EM_LATTICEMICO32: Elf_Half = 138; /// RISC processor for Lattice FPGA architecture
pub const EM_SE_C17:        Elf_Half = 139; /// Seiko Epson C17 family
pub const EM_TI_C6000:      Elf_Half = 140; /// The Texas Instruments TMS320C6000 DSP family
pub const EM_TI_C2000:      Elf_Half = 141; /// The Texas Instruments TMS320C2000 DSP family
pub const EM_TI_C5500:      Elf_Half = 142; /// The Texas Instruments TMS320C55x DSP family
pub const EM_TI_ARP32:      Elf_Half = 143; /// Texas Instruments Application Specific RISC Processor, 32bit fetch
pub const EM_TI_PRU:        Elf_Half = 144; /// Texas Instruments Programmable Realtime Unit
pub const EM_MMDSP_PLUS:    Elf_Half = 160; /// STMicroelectronics 64bit VLIW Data Signal Processor
pub const EM_CYPRESS_M8C:   Elf_Half = 161; /// Cypress M8C microprocessor
pub const EM_R32C:          Elf_Half = 162; /// Renesas R32C series microprocessors
pub const EM_TRIMEDIA:      Elf_Half = 163; /// NXP Semiconductors TriMedia architecture family
pub const EM_QDSP6:         Elf_Half = 164; /// QUALCOMM DSP6 Processor
pub const EM_8051:          Elf_Half = 165; /// Intel 8051 and variants
pub const EM_STXP7X:        Elf_Half = 166; /// STMicroelectronics STxP7x family of configurable and extensible RISC processors
pub const EM_NDS32:         Elf_Half = 167; /// Andes Technology compact code size embedded RISC processor family
pub const EM_ECOG1:         Elf_Half = 168; /// Cyan Technology eCOG1X family
pub const EM_ECOG1X:        Elf_Half = 168; /// Cyan Technology eCOG1X family
pub const EM_MAXQ30:        Elf_Half = 169; /// Dallas Semiconductor MAXQ30 Core Micro-controllers
pub const EM_XIMO16:        Elf_Half = 170; /// New Japan Radio (NJR) 16-bit DSP Processor
pub const EM_MANIK:         Elf_Half = 171; /// M2000 Reconfigurable RISC Microprocessor
pub const EM_CRAYNV2:       Elf_Half = 172; /// Cray Inc. NV2 vector architecture
pub const EM_RX:            Elf_Half = 173; /// Renesas RX family
pub const EM_METAG:         Elf_Half = 174; /// Imagination Technologies META processor architecture
pub const EM_MCST_ELBRUS:   Elf_Half = 175; /// MCST Elbrus general purpose hardware architecture
pub const EM_ECOG16:        Elf_Half = 176; /// Cyan Technology eCOG16 family
pub const EM_CR16:          Elf_Half = 177; /// National Semiconductor CompactRISC CR16 16-bit microprocessor
pub const EM_ETPU:          Elf_Half = 178; /// Freescale Extended Time Processing Unit
pub const EM_SLE9X:         Elf_Half = 179; /// Infineon Technologies SLE9X core
pub const EM_L10M:          Elf_Half = 180; /// Intel L10M
pub const EM_K10M:          Elf_Half = 181; /// Intel K10M
pub const EM_AARCH64:       Elf_Half = 183; /// ARM 64-bit architecture (AARCH64)
pub const EM_AVR32:         Elf_Half = 185; /// Atmel Corporation 32-bit microprocessor family
pub const EM_STM8:          Elf_Half = 186; /// STMicroeletronics STM8 8-bit microcontroller
pub const EM_TILE64:        Elf_Half = 187; /// Tilera TILE64 multicore architecture family
pub const EM_TILEPRO:       Elf_Half = 188; /// Tilera TILEPro multicore architecture family
pub const EM_MICROBLAZE:    Elf_Half = 189; /// Xilinx MicroBlaze 32-bit RISC soft processor core
pub const EM_CUDA:          Elf_Half = 190; /// NVIDIA CUDA architecture
pub const EM_TILEGX:        Elf_Half = 191; /// Tilera TILE-Gx multicore architecture family
pub const EM_CLOUDSHIELD:   Elf_Half = 192; /// CloudShield architecture family
pub const EM_COREA_1ST:     Elf_Half = 193; /// KIPO-KAIST Core-A 1st generation processor family
pub const EM_COREA_2ND:     Elf_Half = 194; /// KIPO-KAIST Core-A 2nd generation processor family
pub const EM_ARC_COMPACT2:  Elf_Half = 195; /// Synopsys ARCompact V2
pub const EM_OPEN8:         Elf_Half = 196; /// Open8 8-bit RISC soft processor core
pub const EM_RL78:          Elf_Half = 197; /// Renesas RL78 family
pub const EM_VIDEOCORE5:    Elf_Half = 198; /// Broadcom VideoCore V processor
pub const EM_78KOR:         Elf_Half = 199; /// Renesas 78KOR family
pub const EM_56800EX:       Elf_Half = 200; /// Freescale 56800EX Digital Signal Controller (DSC)
pub const EM_BA1:           Elf_Half = 201; /// Beyond BA1 CPU architecture
pub const EM_BA2:           Elf_Half = 202; /// Beyond BA2 CPU architecture
pub const EM_XCORE:         Elf_Half = 203; /// XMOS xCORE processor family
pub const EM_MCHP_PIC:      Elf_Half = 204; /// Microchip 8-bit PIC(r) family
pub const EM_INTEL205:      Elf_Half = 205; /// Reserved by Intel
pub const EM_INTEL206:      Elf_Half = 206; /// Reserved by Intel
pub const EM_INTEL207:      Elf_Half = 207; /// Reserved by Intel
pub const EM_INTEL208:      Elf_Half = 208; /// Reserved by Intel
pub const EM_INTEL209:      Elf_Half = 209; /// Reserved by Intel
pub const EM_KM32:          Elf_Half = 210; /// KM211 KM32 32-bit processor
pub const EM_KMX32:         Elf_Half = 211; /// KM211 KMX32 32-bit processor
pub const EM_KMX16:         Elf_Half = 212; /// KM211 KMX16 16-bit processor
pub const EM_KMX8:          Elf_Half = 213; /// KM211 KMX8 8-bit processor
pub const EM_KVARC:         Elf_Half = 214; /// KM211 KVARC processor
pub const EM_CDP:           Elf_Half = 215; /// Paneve CDP architecture family
pub const EM_COGE:          Elf_Half = 216; /// Cognitive Smart Memory Processor
pub const EM_COOL:          Elf_Half = 217; /// Bluechip Systems CoolEngine
pub const EM_NORC:          Elf_Half = 218; /// Nanoradio Optimized RISC
pub const EM_CSR_KALIMBA:   Elf_Half = 219; /// CSR Kalimba architecture family
pub const EM_Z80:           Elf_Half = 220; /// Zilog Z80
pub const EM_VISIUM:        Elf_Half = 221; /// Controls and Data Services VISIUMcore processor
pub const EM_FT32:          Elf_Half = 222; /// FTDI Chip FT32 high performance 32-bit RISC architecture
pub const EM_MOXIE:         Elf_Half = 223; /// Moxie processor family
pub const EM_AMDGPU:        Elf_Half = 224; /// AMD GPU architecture
pub const EM_RISCV:         Elf_Half = 243; /// RISC-V

// e_version
pub const EV_NONE:    Elf_Word = 0; /// Invalid version
pub const EV_CURRENT: Elf_Word = 1; /// Current version


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
pub const SHT_NULL:          Elf_Word = 0;
pub const SHT_PROGBITS:      Elf_Word = 1;
pub const SHT_SYMTAB:        Elf_Word = 2;
pub const SHT_STRTAB:        Elf_Word = 3;
pub const SHT_RELA:          Elf_Word = 4;
pub const SHT_HASH:          Elf_Word = 5;
pub const SHT_DYNAMIC:       Elf_Word = 6;
pub const SHT_NOTE:          Elf_Word = 7;
pub const SHT_NOBITS:        Elf_Word = 8;
pub const SHT_REL:           Elf_Word = 9;
pub const SHT_SHLIB:         Elf_Word = 10;
pub const SHT_DYNSYM:        Elf_Word = 11;
pub const SHT_INIT_ARRAY:    Elf_Word = 14;
pub const SHT_FINI_ARRAY:    Elf_Word = 15;
pub const SHT_PREINIT_ARRAY: Elf_Word = 16;
pub const SHT_GROUP:         Elf_Word = 17;
pub const SHT_SYMTAB_SHNDX:  Elf_Word = 18;
pub const SHT_LOOS:          Elf_Word = 0x60000000;
pub const SHT_HIOS:          Elf_Word = 0x6fffffff;
pub const SHT_LOPROC:        Elf_Word = 0x70000000;
pub const SHT_HIPROC:        Elf_Word = 0x7fffffff;
pub const SHT_LOUSER:        Elf_Word = 0x80000000;
pub const SHT_HIUSER:        Elf_Word = 0xffffffff;

// sh_flags
pub const SHF_WRITE:            Elf_Word = 0x1;
pub const SHF_ALLOC:            Elf_Word = 0x2;
pub const SHF_EXECINSTR:        Elf_Word = 0x4;
pub const SHF_MERGE:            Elf_Word = 0x10;
pub const SHF_STRINGS:          Elf_Word = 0x20;
pub const SHF_INFO_LINK:        Elf_Word = 0x40;
pub const SHF_LINK_ORDER:       Elf_Word = 0x80;
pub const SHF_OS_NONCONFORMING: Elf_Word = 0x100;
pub const SHF_GROUP:            Elf_Word = 0x200;
pub const SHF_TLS:              Elf_Word = 0x400;
pub const SHF_COMPRESSED:       Elf_Word = 0x800;
pub const SHF_MASKOS:           Elf_Word = 0x0ff00000;
pub const SHF_MASKPROC:         Elf_Word = 0xf0000000;

// ch_type
pub const ELFCOMPRESS_ZLIB:  Elf_Word  = 1;
pub const ELFCOMPRESS_LOOS:  Elf_Word  = 0x60000000;
pub const ELFCOMPRESS_HIOS:  Elf_Word  = 0x6fffffff;
pub const ELFCOMPRESS_LOPROC: Elf_Word = 0x70000000;
pub const ELFCOMPRESS_HIPROC: Elf_Word = 0x7fffffff;

// Section group flags
pub const GRP_COMDAT:  Elf_Word  = 0x1;
pub const GRP_MASKOS:  Elf_Word  = 0x0ff00000;
pub const GRP_MASKPROC: Elf_Word = 0xf0000000;


// Symbol table
pub const STN_UNDEF: Elf_Half = 0;

// st_info
pub const STB_LOCAL:  u8 = 0;
pub const STB_GLOBAL: u8 = 1;
pub const STB_WEAK:   u8 = 2;
pub const STB_LOOS:   u8 = 10;
pub const STB_HIOS:   u8 = 12;
pub const STB_LOPROC: u8 = 13;
pub const STB_HIPROC: u8 = 15;

pub const STT_NOTYPE:  u8 = 0;
pub const STT_OBJECT:  u8 = 1;
pub const STT_FUNC:    u8 = 2;
pub const STT_SECTION: u8 = 3;
pub const STT_FILE:    u8 = 4;
pub const STT_COMMON:  u8 = 5;
pub const STT_TLS:     u8 = 6;
pub const STT_LOOS:    u8 = 10;
pub const STT_HIOS:    u8 = 12;
pub const STT_LOPROC:  u8 = 13;
pub const STT_HIPROC:  u8 = 15;

// st_other
pub const STV_DEFAULT:   u8 = 0;
pub const STV_INTERNAL:  u8 = 1;
pub const STV_HIDDEN:    u8 = 2;
pub const STV_PROTECTED: u8 = 3;

