//! Traits to represent `repr`s
//!
//! If it is important for a generic parameter to have a particular `repr`, you can use
//! the traits in this crate to ensure that it has the needed `repr`.
//!
//! For example, if you have an `unsafe` function that requires a specific `repr`,
//! you can use these traits to create a safe wrapper around it.
//!
//! ```rust
//! use repr_trait::Packed;
//!
//! // Safety: Only safe to call when T has #[repr(packed)]
//! unsafe fn safe_when_packed<T>(_param: T) {
//!     unimplemented!()
//! }
//!
//! fn safe_wrapper<T: Packed>(param: T) {
//!     // Safety: Safe because T is guaranteed to be #[repr(packed)]
//!     unsafe {
//!         safe_when_packed(param)
//!     }
//! }
//! ```
//!
//! Implementing the traits from this crate is easy using derive macros. There is a derive
//! macro for each included trait.
//!
//! ```rust
//! # use repr_trait::Packed;
//! # fn safe_wrapper<T: Packed>(param: T) { param; }
//! #[derive(Packed, Default)]
//! #[repr(packed)]
//! struct PackedData(u32, u8);
//!
//! safe_wrapper(PackedData(123, 45));
//! ```
//!
//! If the appropriate `repr` is not specified, the derive macro will refuse to compile.
//!
//! ```rust,compile_fail
//! #[derive(Packed)]
//! struct NotPacked(u32, u8);
//! ```
use std::ptr::NonNull;

macro_rules! trait_and_docs {
    ($tr:ident as $repr:expr) => {
        trait_and_docs!(@
            $tr
            trait_doc concat!("Trait for types declared with `#[repr(", $repr, ")]`."),
            derive_doc concat!("Derive macro for [`", stringify!($tr), "`](trait@", stringify!($tr), ")")
        );
    };
    (@ $tr:ident trait_doc $trait_doc:expr, derive_doc $derive_doc:expr) => {
        #[doc = $trait_doc]
        ///
        /// # Safety
        ///
        /// This trait should only be implemented for types with the correct `repr`. Because `repr`s
        /// cannot be checked by the compiler, this trait is `unsafe`.
        ///
        /// Use the corresponding derive macro to safely derive this on any type with the correct
        /// `repr`.
        pub unsafe trait $tr {}

        #[doc = $derive_doc]
        ///
        /// Can be added to any type with the correct
        pub use repr_trait_derive::$tr;
    }

}

trait_and_docs!(C as "C");
trait_and_docs!(Packed as "packed");
trait_and_docs!(Transparent as "transparent");

/// Trait for types declared with #[repr(u*)] or #[repr(i*)].
///
/// # Safety
///
/// This trait should only be implemented for types with the correct `repr`. Because `repr`s
/// cannot be checked by the compiler, this trait is `unsafe`.
///
/// Accessing the discriminant value should happen through safe [`discriminant`] function.
/// This functionality is not implemented as a method on a trait, but as a function to
/// ensure that it's implementation cannot be changed in an unlikely event of
/// implementing this trait manually.
pub unsafe trait PrimitiveRepr {
    type Type;
}

/// Reads a discriminant of an enum with a primitive representation.
pub fn discriminant<T: PrimitiveRepr>(enum_value: &T) -> &T::Type {
    // SAFETY: Proc macro ensures that `repr(u*)` or `repr(i*)` is present.

    // Taken from here: https://doc.rust-lang.org/stable/std/mem/fn.discriminant.html#accessing-the-numeric-value-of-the-discriminant
    let discriminant_ptr = NonNull::from(enum_value).cast::<T::Type>();
    unsafe { discriminant_ptr.as_ref() }
}

/// Derive macro for [`PrimitiveRepr`](trait@PrimitiveRepr)
pub use repr_trait_derive::PrimitiveRepr;

#[cfg(test)]
mod test {
    // Due to https://github.com/dtolnay/trybuild/issues/58, all trybuild tests must
    // run in the same test case.
    #[test]
    fn test_compilation() {
        let t = trybuild::TestCases::new();
        t.compile_fail("test/packed_fail.rs");
        t.pass("test/packed_pass.rs");

        t.compile_fail("test/transparent_fail.rs");
        t.pass("test/transparent_pass.rs");

        t.compile_fail("test/c_fail.rs");
        t.pass("test/c_pass.rs");

        t.compile_fail("test/primitive_repr_fail.rs");

        t.compile_fail("test/zero_variants_fail.rs");
    }
}
