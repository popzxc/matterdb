//! This crate provides macros for deriving some useful methods and traits for the MatterDB.

#![recursion_limit = "128"]
#![deny(unsafe_code, bare_trait_objects)]
#![warn(missing_docs, missing_debug_implementations)]

extern crate proc_macro;

mod db_traits;

use proc_macro::TokenStream;
use syn::{Attribute, NestedMeta};

/// Derives `BinaryValue` trait. The target type must implement (de)serialization logic,
/// which should be provided externally.
///
/// The trait currently supports the following codecs:
///
/// - `bincode` serialization via the eponymous crate. Switched on by the
///   `#[binary_value(codec = "bincode")]` attribute.
///
/// # Container Attributes
///
/// ## `codec`
///
/// Selects the serialization codec to use. Allowed values are `protobuf` (used by default)
/// and `bincode`.
///
/// # Examples
///
/// With Protobuf serialization:
///
/// With `bincode` serialization:
///
/// ```ignore
/// #[derive(Clone, Debug, Serialize, Deserialize, BinaryValue)]
/// #[binary_value(codec = "bincode")]
/// pub struct Wallet {
///     pub username: PublicKey,
///     /// Current balance of the wallet.
///     pub balance: u64,
/// }
///
/// let wallet = Wallet {
///     username: "Alice".to_owned(),
///     balance: 100,
/// };
/// let bytes = wallet.to_bytes();
/// ```
#[proc_macro_derive(BinaryValue, attributes(binary_value))]
pub fn binary_value(input: TokenStream) -> TokenStream {
    db_traits::impl_binary_value(input)
}

/// Derives `FromAccess` trait.
///
/// This macro can be applied only to `struct`s, each field of which implements `FromAccess`
/// itself (e.g., indexes, `Group`s, or `Lazy` indexes). The macro instantiates each field
/// using the address created by appending a dot `.` and the name of the field or its override
/// (see [below](#rename)) to the root address where the struct is created. For example,
/// if the struct is created at the address `"foo"` and has fields `"list"` and `"map"`, they
/// will be instantiated at addresses `"foo.list"` and `"foo.map"`, respectively.
///
/// The struct must have at least one type param, which will correspond to the `Access` type.
/// The derive logic will determine this param as the first param with `T: Access` bound.
/// If there are no such params, but there is a single type param, it will be used.
///
/// # Container Attributes
///
/// ## `transparent`
///
/// ```text
/// #[from_access(transparent)]`
/// ```
///
/// Switches to the *transparent* layout similarly to `#[repr(transparent)]`
/// or `#[serde(transparent)]`.
/// A struct with the transparent layout must have a single field. The field will be created at
/// the same address as the struct itself (i.e., no suffix will be added).
///
/// # Field Attributes
///
/// ## `rename`
///
/// ```text
/// #[from_access(rename = "name")]
/// ```
///
/// Changes the suffix appended to the address when creating a field. The name should follow
/// conventions for index names.
#[proc_macro_derive(FromAccess, attributes(from_access))]
pub fn from_access(input: TokenStream) -> TokenStream {
    db_traits::impl_from_access(input)
}

pub(crate) fn find_meta_attrs(name: &str, args: &[Attribute]) -> Option<NestedMeta> {
    args.as_ref()
        .iter()
        .filter_map(|a| a.parse_meta().ok())
        .find(|m| m.path().is_ident(name))
        .map(NestedMeta::from)
}
