use super::Elf_Word;

#[derive(Debug, Clone, Copy, Fail)]
pub enum ElfError {
    #[fail(display = "not an ELF file")]
    NotElfFile,

    #[fail(display = "invalid {} header field {}={}", header, field, value)]
    InvalidHeaderField { header: &'static str, field: &'static str, value: u64 },

    #[fail(display = "size error: expected={}, actual={}", expected, actual)]
    SizeError { expected: usize, actual: usize },

    #[fail(display = "alignment error: alignment={}, address={}", alignment, address)]
    AlignmentError { alignment: usize, address: usize },

    #[fail(display = "not a multiple of size: size={}, length={}", size, length)]
    NotMultipleOfSize { size: usize, length: usize },

    #[fail(display = "index out of bounds: index={}, length={}", index, length)]
    IndexOutOfBounds { index: usize, length: usize },

    #[fail(display = "{} {} not contained in file", what, which)]
    NotContainedInFile { what: &'static str, which: u64 },

    #[fail(display = "multiple {} sections", section)]
    MultipleSections { section: &'static str },

    #[fail(display = "invalid linked section={}", linked)]
    InvalidLinkedSection { linked: Elf_Word },

    #[fail(display = "invalid section type: expected={}, actual={}", expected, actual)]
    InvalidSectionType { expected: Elf_Word, actual: Elf_Word },

    #[fail(display = "{}", msg)]
    Msg { msg: &'static str },
}

pub type ElfResult<T> = Result<T, ElfError>;
