// Copyright 2023, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE

use super::{SizedStorageType, StorageCache, StorageGuard, StorageGuardMut, StorageType};
use crate::crypto;
use alloy_primitives::U256;
use std::{cell::OnceCell, marker::PhantomData, slice::SliceIndex};

/// Accessor for a storage-backed vector
pub struct StorageVec<S: StorageType> {
    slot: U256,
    base: OnceCell<U256>,
    marker: PhantomData<S>,
}

impl<S: StorageType> StorageType for StorageVec<S> {
    fn new(slot: U256, offset: u8) -> Self {
        debug_assert!(offset == 0);
        Self {
            slot,
            base: OnceCell::new(),
            marker: PhantomData,
        }
    }
}

impl<S: StorageType> StorageVec<S> {
    /// Returns `true` if the collection contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the number of elements stored.
    pub fn len(&self) -> usize {
        let word: U256 = StorageCache::get_word(self.slot).into();
        word.try_into().unwrap()
    }

    /// Overwrites the vector's length.
    ///
    /// # Safety
    ///
    /// It must be sensible to create accessors for `S` from zero-slots,
    /// or any junk data left over from previous dirty removal operations such as [`StorageVec::pop`].
    /// Note that `StorageVec` has unlimited capacity, so all lengths are valid.
    pub unsafe fn set_len(&mut self, len: usize) {
        StorageCache::set_word(self.slot, U256::from(len).into())
    }

    /// Gets an accessor to the element at a given index, if it exists.
    /// Note: the accessor is protected by a [`StoreageGuard`], which restricts
    /// its lifetime to that of `&self`.
    pub fn getter<I>(&self, index: I) -> Option<StorageGuard<S>>
    where
        I: SliceIndex<[S]> + TryInto<usize>,
    {
        let store = unsafe { self.get_raw(index)? };
        Some(StorageGuard::new(store))
    }

    /// Gets a mutable accessor to the element at a given index, if it exists.
    /// Note: the accessor is protected by a [`StoreageGuardMut`], which restricts
    /// its lifetime to that of `&mut self`.
    pub fn setter<I>(&mut self, index: I) -> Option<StorageGuardMut<S>>
    where
        I: SliceIndex<[S]> + TryInto<usize>,
    {
        let store = unsafe { self.get_raw(index)? };
        Some(StorageGuardMut::new(store))
    }

    /// Gets the underlying accessor to the element at a given index, if it exists.
    ///
    /// # Safety
    ///
    /// Because the accessor is unconstrained by a storage guard, storage aliasing is possible
    /// if used incorrectly. Two or more mutable references to the same `S` are possible, as are
    /// read-after-write scenarios.
    pub unsafe fn get_raw<I>(&self, index: I) -> Option<S>
    where
        I: SliceIndex<[S]> + TryInto<usize>,
    {
        let index = index.try_into().ok()?;
        let width = S::SIZE as usize;

        if index > self.len() {
            return None;
        }

        let density = 32 / width;
        let offset = self.base() + U256::from(width * index / density);
        Some(S::new(offset, (index % density) as u8))
    }

    /// Like [`std::Vec::push`], but returns a mutable accessor to the new slot.
    /// This enables pushing elements without constructing them first.
    ///
    /// # Example
    ///
    /// ```
    /// use stylus_sdk::storage::{StorageVec, StorageType, StorageU256};
    /// use stylus_sdk::alloy_primitives::U256;
    ///
    /// let mut vec: StorageVec<StorageVec<StorageU256>> = StorageVec::new(U256::ZERO, 0);
    /// let mut inner_vec = vec.open();
    /// inner_vec.push(U256::from(8));
    ///
    /// let value = inner_vec.get(0).unwrap();
    /// assert_eq!(value.get(), U256::from(8));
    /// assert_eq!(inner_vec.len(), 1);
    /// ```
    pub fn open(&mut self) -> StorageGuardMut<S> {
        let index = self.len();
        let width = S::SIZE as usize;
        unsafe { self.set_len(index) };

        let density = 32 / width;
        let offset = self.base() + U256::from(width * index / density);
        let store = S::new(offset, (index % density) as u8);
        StorageGuardMut::new(store)
    }

    /// Removes and returns the last element of the vector, if any.
    pub fn pop(&mut self) -> Option<S> {
        let index = match self.len() {
            0 => return None,
            x => x - 1,
        };
        let item = unsafe { self.get_raw(index) };
        StorageCache::set_word(self.slot, U256::from(index).into());
        item
    }

    /// Shortens the vector, keeping the first `len` elements.
    /// Note: this method does not clear any underlying storage.
    pub fn truncate(&mut self, len: usize) {
        if len < self.len() {
            // SAFETY: operation leaves only existing values
            unsafe { self.set_len(len) }
        }
    }

    /// Determines where in storage indices start. Could be made const in the future.
    fn base(&self) -> &U256 {
        self.base
            .get_or_init(|| crypto::keccak(self.slot.to_be_bytes::<32>()).into())
    }
}

impl<S: SizedStorageType> StorageVec<S> {
    /// Adds an element to the end of the vector.
    pub fn push(&mut self, value: S::Value) {
        let mut store = self.open();
        store.set_exact(value);
    }
}
