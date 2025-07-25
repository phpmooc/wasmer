/*
 * ! Remove me once rkyv generates doc-comments for fields or generates an #[allow(missing_docs)]
 * on their own.
 */
#![allow(missing_docs)]

//! This module define the required structures to emit custom
//! Sections in a `Compilation`.
//!
//! The functions that access a custom [`CustomSection`] would need
//! to emit a custom relocation: `RelocationTarget::CustomSection`, so
//! it can be patched later by the engine (native or JIT).

use super::relocation::{ArchivedRelocation, Relocation, RelocationLike};
use crate::lib::std::vec::Vec;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};
use wasmer_types::entity_impl;

/// Index type of a Section defined inside a WebAssembly `Compilation`.
#[derive(
    RkyvSerialize,
    RkyvDeserialize,
    Archive,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Debug,
    Default,
)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "artifact-size", derive(loupe::MemoryUsage))]
#[rkyv(derive(Debug, Hash, PartialEq, Eq), compare(PartialEq, PartialOrd))]
pub struct SectionIndex(u32);

entity_impl!(SectionIndex);

/// Custom section Protection.
///
/// Determines how a custom section may be used.
#[cfg_attr(feature = "artifact-size", derive(loupe::MemoryUsage))]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(RkyvSerialize, RkyvDeserialize, Archive, Debug, Clone, PartialEq, Eq)]
#[rkyv(derive(Debug), compare(PartialEq, PartialOrd))]
#[repr(u8)]
pub enum CustomSectionProtection {
    /// A custom section with read permission.
    Read,

    /// A custom section with read and execute permissions.
    ReadExecute,
}

/// A Section for a `Compilation`.
///
/// This is used so compilers can store arbitrary information
/// in the emitted module.
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "artifact-size", derive(loupe::MemoryUsage))]
#[derive(RkyvSerialize, RkyvDeserialize, Archive, Debug, Clone, PartialEq, Eq)]
#[rkyv(derive(Debug), compare(PartialEq))]
pub struct CustomSection {
    /// Memory protection that applies to this section.
    pub protection: CustomSectionProtection,

    /// Alignment of this section. When missing, the default value for
    /// each platform shall be used.
    pub alignment: Option<u64>,

    /// The bytes corresponding to this section.
    ///
    /// > Note: These bytes have to be at-least 8-byte aligned
    /// > (the start of the memory pointer).
    /// > We might need to create another field for alignment in case it's
    /// > needed in the future.
    pub bytes: SectionBody,

    /// Relocations that apply to this custom section.
    pub relocations: Vec<Relocation>,
}

/// Any struct that acts like a `CustomSection`.
#[allow(missing_docs)]
pub trait CustomSectionLike<'a> {
    type Relocations: RelocationLike;

    fn protection(&self) -> CustomSectionProtection;
    fn alignment(&self) -> Option<u64>;
    fn bytes(&self) -> &[u8];
    fn relocations(&'a self) -> &'a [Self::Relocations];
}

impl<'a> CustomSectionLike<'a> for CustomSection {
    type Relocations = Relocation;

    fn protection(&self) -> CustomSectionProtection {
        self.protection.clone()
    }

    fn alignment(&self) -> Option<u64> {
        self.alignment
    }

    fn bytes(&self) -> &[u8] {
        self.bytes.0.as_ref()
    }

    fn relocations(&'a self) -> &'a [Self::Relocations] {
        self.relocations.as_slice()
    }
}

impl<'a> CustomSectionLike<'a> for ArchivedCustomSection {
    type Relocations = ArchivedRelocation;

    fn protection(&self) -> CustomSectionProtection {
        let protection = rkyv::deserialize::<CustomSectionProtection, ()>(&self.protection);
        protection.unwrap()
    }

    fn alignment(&self) -> Option<u64> {
        let alignment = rkyv::deserialize::<Option<u64>, ()>(&self.alignment);
        alignment.unwrap()
    }

    fn bytes(&self) -> &[u8] {
        self.bytes.0.as_ref()
    }

    fn relocations(&'a self) -> &'a [Self::Relocations] {
        self.relocations.as_slice()
    }
}

/// The bytes in the section.
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "artifact-size", derive(loupe::MemoryUsage))]
#[derive(RkyvSerialize, RkyvDeserialize, Archive, Debug, Clone, PartialEq, Eq, Default)]
#[rkyv(derive(Debug), compare(PartialEq, PartialOrd))]
pub struct SectionBody(#[cfg_attr(feature = "enable-serde", serde(with = "serde_bytes"))] Vec<u8>);

impl SectionBody {
    /// Create a new section body with the given contents.
    pub fn new_with_vec(contents: Vec<u8>) -> Self {
        Self(contents)
    }

    /// Returns a raw pointer to the section's buffer.
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Returns the length of this section in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Dereferences into the section's buffer.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Returns whether or not the section body is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl ArchivedSectionBody {
    /// Returns the length of this section in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether or not the section body is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
