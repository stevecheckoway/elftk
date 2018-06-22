use std::{iter, slice};

#[derive(Debug, Clone)]
pub enum ElfT<T32, T64, E> {
    Elf32LE(T32, E),
    Elf32BE(T32, E),
    Elf64LE(T64, E),
    Elf64BE(T64, E),
}

impl<T32, T64, E> ElfT<T32, T64, E> {
    #[inline]
    pub(super) fn extra(&self) -> &E {
        match *self {
            ElfT::Elf32LE(_, ref e) |
            ElfT::Elf32BE(_, ref e) |
            ElfT::Elf64LE(_, ref e) |
            ElfT::Elf64BE(_, ref e) => &e,
        }
    }

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
pub type ElfSliceRef<'a, T32, T64, E> = ElfT<&'a [T32], &'a [T64], E>;

impl<'a, T32, T64, E: Clone> ElfSliceRef<'a, T32, T64, E> {
    pub fn len(&self) -> usize {
        self.apply(|s| s.len(), |s| s.len())
    }

    pub fn get(&self, index: usize) -> Option<ElfRef<'a, T32, T64, E>> {
        if index >= self.len() {
            return None;
        }
        Some(self.clone().map(move |slice| &slice[index], move |slice| &slice[index]))
    }

    pub fn iter(&self) -> ElfIter<'a, T32, T64, E> {
        self.clone().map(move |slice| slice.iter(), move |slice| slice.iter())
    }
}

impl<'a, T32, T64, E: 'a + Clone> iter::IntoIterator for ElfSliceRef<'a, T32, T64, E> {
    type Item = ElfRef<'a, T32, T64, E>;
    type IntoIter = ElfIter<'a, T32, T64, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.map(move |slice| slice.iter(), move |slice| slice.iter())
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

