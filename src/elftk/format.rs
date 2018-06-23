use std::{iter, mem, slice};

use super::ElfStruct;

#[derive(Debug, Clone)]
pub enum ElfT<T32, T64, E> {
    Elf32LE(T32, E),
    Elf32BE(T32, E),
    Elf64LE(T64, E),
    Elf64BE(T64, E),
}

impl<T32, T64, E> ElfT<T32, T64, E> {
    /// Construct a new `ElfT` with the same format (32- or 64-bit, little- or bit-endian) by using
    /// `f1` to construct the 32-bit value or `f2` to construct the 64-bit value and with the given
    /// extra value `e`.
    ///
    /// Both `f1` and `f2` must return values suitable for both little- and big-endian.
    #[inline]
    pub(super) fn construct_from<R32, R64, F1, F2, E2>(&self, f1: F1, f2: F2, e: E2) -> ElfT<R32, R64, E2> where
        F1: FnOnce() -> R32,
        F2: FnOnce() -> R64,
    {
        match *self {
            ElfT::Elf32LE(..) => ElfT::Elf32LE(f1(), e),
            ElfT::Elf32BE(..) => ElfT::Elf32BE(f1(), e),
            ElfT::Elf64LE(..) => ElfT::Elf64LE(f2(), e),
            ElfT::Elf64BE(..) => ElfT::Elf64BE(f2(), e),
        }
    }

    /// Return the common extra parameter.
    #[inline]
    pub(super) fn extra(&self) -> &E {
        match *self {
            ElfT::Elf32LE(_, ref e) |
            ElfT::Elf32BE(_, ref e) |
            ElfT::Elf64LE(_, ref e) |
            ElfT::Elf64BE(_, ref e) => &e,
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
            ElfT::Elf32LE(ref x, _) |
            ElfT::Elf32BE(ref x, _) => f1(&x),
            ElfT::Elf64LE(ref x, _) |
            ElfT::Elf64BE(ref x, _) => f2(&x),
        }
    }

    /// Transform `self` into a new `ElfT` with the same extra data by applying `f1` to 32-bit
    /// types or `f2` to 64-bit types.
    ///
    /// Both `f1` and `f2` must behave correctly regardless of the underlying endianness.
    #[inline]
    pub(super) fn map<R32, R64, F1, F2>(self, f1: F1, f2: F2) -> ElfT<R32, R64, E> where
        F1: FnOnce(T32) -> R32,
        F2: FnOnce(T64) -> R64,
    {
        match self {
            ElfT::Elf32LE(x, e) => ElfT::Elf32LE(f1(x), e),
            ElfT::Elf32BE(x, e) => ElfT::Elf32BE(f1(x), e),
            ElfT::Elf64LE(x, e) => ElfT::Elf64LE(f2(x), e),
            ElfT::Elf64BE(x, e) => ElfT::Elf64BE(f2(x), e),
        }
    }
}

pub type ElfFormat = ElfT<(), (), ()>;

#[cfg(all(target_pointer_width = "32", target_endian = "little"))]
pub const NATIVE_FILE_FORMAT: ElfFormat = ElfT::Elf32LE((), ());

#[cfg(all(target_pointer_width = "32", target_endian = "big"))]
pub const NATIVE_FILE_FORMAT: ElfFormat = ElfT::Elf32BE((), ());

#[cfg(all(target_pointer_width = "64", target_endian = "little"))]
pub const NATIVE_FILE_FORMAT: ElfFormat = ElfT::Elf64LE((), ());

#[cfg(all(target_pointer_width = "64", target_endian = "big"))]
pub const NATIVE_FILE_FORMAT: ElfFormat = ElfT::Elf64LE((), ());

pub type ElfRef<'a, T32, T64, E> = ElfT<&'a T32, &'a T64, E>;

impl<'a, T32, T64, E> ElfRef<'a, T32, T64, E> where
    T32: ElfStruct,
    T64: ElfStruct,
{
    pub(super) fn try_from(raw: ElfSliceRef<'a, u8, u8, E>) -> Option<Self> {
        let expected_len = raw.apply(|_| mem::size_of::<T32>(), |_| mem::size_of::<T64>());
        if expected_len != raw.len() {
            return None;
        }
        Some(raw.map(|slice| unsafe { &*(slice.as_ptr() as *const T32) },
                     |slice| unsafe { &*(slice.as_ptr() as *const T64) }))
    }
}

pub type ElfSliceRef<'a, T32, T64, E> = ElfT<&'a [T32], &'a [T64], E>;

impl<'a, T32, T64, E> ElfSliceRef<'a, T32, T64, E> {
    pub fn len(&self) -> usize {
        self.apply(|s| s.len(), |s| s.len())
    }
}

impl<'a, T32, T64, E: Clone> ElfSliceRef<'a, T32, T64, E> {
    pub fn get(&self, index: usize) -> Option<ElfRef<'a, T32, T64, E>> {
        if index >= self.len() {
            return None;
        }
        Some(self.clone().map(move |slice| &slice[index], move |slice| &slice[index]))
    }

    pub fn iter(&self) -> ElfIter<'a, T32, T64, E> {
        self.clone().map(move |slice| slice.into_iter(), move |slice| slice.into_iter())
    }
}

impl<'a, T32, T64, E> ElfSliceRef<'a, T32, T64, E> where
    T32: ElfStruct,
    T64: ElfStruct,
{
    /// Try to convert from a reference to a `&[u8]` to a reference to a `&[T32]` or `&[T64]`.
    ///
    /// The `raw` parameter must reference a correctly aligned slice whose length is a multiple of
    /// the size of `T32` or `T64`.
    pub(super) fn try_from<'b: 'a>(raw: ElfSliceRef<'b, u8, u8, E>) -> Option<Self> {
        if raw.apply(|s| s.len() % mem::size_of::<T32>() != 0,
                     |s| s.len() % mem::size_of::<T64>() != 0) ||
           raw.apply(|s| s.as_ptr() as usize & (mem::align_of::<T32>() - 1) != 0,
                     |s| s.as_ptr() as usize & (mem::align_of::<T64>() - 1) != 0)
        {
            return None;
        }
        let element_len = raw.apply(|_| mem::size_of::<T32>(), |_| mem::size_of::<T64>());
        let num = raw.len() / element_len;
        Some(raw.map(|s| unsafe { slice::from_raw_parts(s.as_ptr() as *const T32, num as usize) },
                     |s| unsafe { slice::from_raw_parts(s.as_ptr() as *const T64, num as usize) }))
    }
}

impl<'a, T32, T64, E: 'a + Clone> iter::IntoIterator for ElfSliceRef<'a, T32, T64, E> {
    type Item = ElfRef<'a, T32, T64, E>;
    type IntoIter = ElfIter<'a, T32, T64, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.map(move |slice| slice.into_iter(), move |slice| slice.into_iter())
    }
}

pub type ElfIter<'a, T32, T64, E> = ElfT<slice::Iter<'a, T32>, slice::Iter<'a, T64>, E>;

impl<'a, T32, T64, E: Clone> Iterator for ElfIter<'a, T32, T64, E> {
    type Item = ElfRef<'a, T32, T64, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            ElfT::Elf32LE(ref mut it, ref e) => it.next().map(move |x| ElfT::Elf32LE(x, e.clone())),
            ElfT::Elf32BE(ref mut it, ref e) => it.next().map(move |x| ElfT::Elf32BE(x, e.clone())),
            ElfT::Elf64LE(ref mut it, ref e) => it.next().map(move |x| ElfT::Elf64LE(x, e.clone())),
            ElfT::Elf64BE(ref mut it, ref e) => it.next().map(move |x| ElfT::Elf64BE(x, e.clone())),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.apply(|it| it.size_hint(), |it| it.size_hint())
    }
}

impl<'a, T32, T64, E> ExactSizeIterator for ElfIter<'a, T32, T64, E> where
    slice::Iter<'a, T32>: ExactSizeIterator,
    slice::Iter<'a, T64>: ExactSizeIterator,
    E: Clone,
{}

impl<'a, T32, T64, E> iter::FusedIterator for ElfIter<'a, T32, T64, E> where
    slice::Iter<'a, T32>: iter::FusedIterator,
    slice::Iter<'a, T64>: iter::FusedIterator,
    E: Clone,
{}

