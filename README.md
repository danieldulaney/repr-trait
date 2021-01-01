Traits to represent `repr`s

If it is important for a generic parameter to have a particular `repr`, you can use
the traits in this crate to ensure that it has the needed `repr`.

For example, if you have an `unsafe` function that requires a specific `repr`,
you can use these traits to create a safe wrapper around it.

```rust
use repr_trait::Packed;

// Safety: Only safe to call when T has #[repr(packed)]
unsafe fn safe_when_packed<T>(_param: T) {
    unimplemented!()
}

fn safe_wrapper<T: Packed>(param: T) {
    // Safety: Safe because T is guaranteed to be #[repr(packed)]
    unsafe {
        safe_when_packed(param)
    }
}
```

Implementing the traits from this crate is easy using derive macros. There is a derive
macro for each included trait.

```rust
#[derive(Packed, Default)]
#[repr(packed)]
struct PackedData(u32, u8);

safe_wrapper(PackedData(123, 45));
```

If the appropriate `repr` is not specified, the derive macro will refuse to compile.

```rust
#[derive(Packed)]
struct NotPacked(u32, u8);
```

# Structure

This repo contains two crates: repr-trait holds the traits and reexports the derive
macros. It is the only trait that should be used in general. repr-trait-derive holds
the derive macro implementations. It is an implementation detail, and should generally
not be relied on directly.
