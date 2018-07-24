use std::{iter, mem, slice};

use types::ElfType;
use error::{ElfResult, ElfError};

pub trait FromEndian {
    fn from_be(x: Self) -> Self;
    fn from_le(x: Self) -> Self;
}

macro_rules! impl_from_endian {
    ($t:ident) => {
        impl FromEndian for $t {
            fn from_be(x: $t) -> $t {
                $t::from_be(x)
            }

            fn from_le(x: $t) -> $t {
                $t::from_le(x)
            }
        }
    }
}

impl_from_endian!(u16);
impl_from_endian!(i16);
impl_from_endian!(u32);
impl_from_endian!(i32);
impl_from_endian!(u64);
impl_from_endian!(i64);

#[derive(Debug, Clone, Copy)]
pub enum ElfT<T32, T64> {
    Elf32LE(T32),
    Elf32BE(T32),
    Elf64LE(T64),
    Elf64BE(T64),
}

impl<T32, T64> ElfT<T32, T64> {
    /// Returns `true` for 64-bit types.
    #[inline]
    pub fn is_64bit(&self) -> bool {
        self.apply(|_| false, |_| true)
    }

    /// Construct a new `ElfT` with the same format (32- or 64-bit, little- or bit-endian) with
    /// `data`.
    #[inline]
    pub(super) fn construct_from<R>(&self, x: R) -> ElfT<R, R> {
        match *self {
            ElfT::Elf32LE(..) => ElfT::Elf32LE(x),
            ElfT::Elf32BE(..) => ElfT::Elf32BE(x),
            ElfT::Elf64LE(..) => ElfT::Elf64LE(x),
            ElfT::Elf64BE(..) => ElfT::Elf64BE(x),
        }
    }

    /// Apply function `f1` to 32-bit types or `f2` to 64-bit types and return the result.
    ///
    /// Both `f1` and `f2` must behave correctly regardless of the underlying endianness.
    #[inline]
    pub(super) fn apply<T, F1, F2>(&self, f1: F1, f2: F2) -> T where
        F1: FnOnce(&T32) -> T,
        F2: FnOnce(&T64) -> T,
    {
        match *self {
            ElfT::Elf32LE(ref x) |
            ElfT::Elf32BE(ref x) => f1(&x),
            ElfT::Elf64LE(ref x) |
            ElfT::Elf64BE(ref x) => f2(&x),
        }
    }

    /// Transform `self` into a new `ElfT` by applying `f1` to 32-bit types or `f2` to 64-bit
    /// types.
    ///
    /// Both `f1` and `f2` must behave correctly regardless of the underlying endianness.
    #[inline]
    pub(super) fn map<R32, R64, F1, F2>(self, f1: F1, f2: F2) -> ElfT<R32, R64> where
        F1: FnOnce(T32) -> R32,
        F2: FnOnce(T64) -> R64,
    {
        match self {
            ElfT::Elf32LE(x) => ElfT::Elf32LE(f1(x)),
            ElfT::Elf32BE(x) => ElfT::Elf32BE(f1(x)),
            ElfT::Elf64LE(x) => ElfT::Elf64LE(f2(x)),
            ElfT::Elf64BE(x) => ElfT::Elf64BE(f2(x)),
        }
    }
}

pub type ElfRef<'a, T32, T64> = ElfT<&'a T32, &'a T64>;

impl<'a, T32, T64> ElfRef<'a, T32, T64> where
    T32: ElfType,
    T64: ElfType,
{
    pub(super) fn try_from(raw: ElfSliceRef<'a, u8, u8>) -> ElfResult<Self> {
        let (e_size, e_align, length, addr) =
            raw.apply(move |s| (mem::size_of::<T32>(), mem::align_of::<T32>(), s.len(), s.as_ptr() as usize),
                      move |s| (mem::size_of::<T64>(), mem::align_of::<T64>(), s.len(), s.as_ptr() as usize));
        if e_size != length {
            return Err(ElfError::SizeError { expected: e_size, actual: length });
        }
        if addr & (e_align-1) != 0 {
            return Err(ElfError::AlignmentError { alignment: e_align, address: addr });
        }
        Ok(raw.map(|slice| unsafe { &*(slice.as_ptr() as *const T32) },
                   |slice| unsafe { &*(slice.as_ptr() as *const T64) }))
    }
}

impl<'a, T32, T64> ElfRef<'a, T32, T64> where
    T32: FromEndian + Copy + Into<T64>,
    T64: FromEndian + Copy,
{
    pub fn get(&self) -> T64 {
        match *self {
            ElfT::Elf32LE(x) => T32::from_le(*x).into(),
            ElfT::Elf32BE(x) => T32::from_be(*x).into(),
            ElfT::Elf64LE(x) => T64::from_le(*x),
            ElfT::Elf64BE(x) => T64::from_be(*x),
        }
    }
}

pub type ElfSliceRef<'a, T32, T64> = ElfT<&'a [T32], &'a [T64]>;

impl<'a, T32, T64> ElfSliceRef<'a, T32, T64> {
    pub fn len(&self) -> usize {
        self.apply(|s| s.len(), |s| s.len())
    }

    pub fn is_empty(&self) -> bool {
        self.apply(|s| s.is_empty(), |s| s.is_empty())
    }

    pub fn get(&self, index: usize) -> ElfResult<ElfRef<'a, T32, T64>> {
        if index >= self.len() {
            return Err(ElfError::IndexOutOfBounds { index, length: self.len() });
        }
        Ok(self.clone().map(move |slice| &slice[index], move |slice| &slice[index]))
    }

    pub fn iter(&self) -> ElfIter<'a, T32, T64> {
        self.clone().map(move |slice| slice.into_iter(), move |slice| slice.into_iter())
    }
}

impl<'a, T32, T64> ElfSliceRef<'a, T32, T64> where
    T32: ElfType,
    T64: ElfType,
{
    /// Try to convert from a reference to a `&[u8]` to a reference to a `&[T32]` or `&[T64]`.
    ///
    /// The `raw` parameter must reference a correctly aligned slice whose length is a multiple of
    /// the size of `T32` or `T64`.
    pub(super) fn try_from<'b: 'a>(raw: ElfSliceRef<'b, u8, u8>) -> ElfResult<Self> {
        let (e_size, e_align, length, addr) =
            raw.apply(move |s| (mem::size_of::<T32>(), mem::align_of::<T32>(), s.len(), s.as_ptr() as usize),
                      move |s| (mem::size_of::<T64>(), mem::align_of::<T64>(), s.len(), s.as_ptr() as usize));
        if length % e_size != 0 {
            return Err(ElfError::NotMultipleOfSize { size: e_size, length });
        }
        if addr & (e_align - 1) != 0 {
            return Err(ElfError::AlignmentError { alignment: e_align, address: addr });
        }
        let num = length / e_size;
        Ok(raw.map(|s| unsafe { slice::from_raw_parts(s.as_ptr() as *const T32, num as usize) },
                   |s| unsafe { slice::from_raw_parts(s.as_ptr() as *const T64, num as usize) }))
    }
}

impl<'a, T32, T64> iter::IntoIterator for ElfSliceRef<'a, T32, T64> {
    type Item = ElfRef<'a, T32, T64>;
    type IntoIter = ElfIter<'a, T32, T64>;

    fn into_iter(self) -> Self::IntoIter {
        self.map(move |slice| slice.into_iter(), move |slice| slice.into_iter())
    }
}

pub type ElfIter<'a, T32, T64> = ElfT<slice::Iter<'a, T32>, slice::Iter<'a, T64>>;

impl<'a, T32, T64> Iterator for ElfIter<'a, T32, T64> {
    type Item = ElfRef<'a, T32, T64>;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            ElfT::Elf32LE(ref mut it) => it.next().map(ElfT::Elf32LE),
            ElfT::Elf32BE(ref mut it) => it.next().map(ElfT::Elf32BE),
            ElfT::Elf64LE(ref mut it) => it.next().map(ElfT::Elf64LE),
            ElfT::Elf64BE(ref mut it) => it.next().map(ElfT::Elf64BE),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.apply(|it| it.size_hint(), |it| it.size_hint())
    }
}

impl<'a, T32, T64> ExactSizeIterator for ElfIter<'a, T32, T64> where
    slice::Iter<'a, T32>: ExactSizeIterator,
    slice::Iter<'a, T64>: ExactSizeIterator,
{}

impl<'a, T32, T64> iter::FusedIterator for ElfIter<'a, T32, T64> where
    slice::Iter<'a, T32>: iter::FusedIterator,
    slice::Iter<'a, T64>: iter::FusedIterator,
{}

