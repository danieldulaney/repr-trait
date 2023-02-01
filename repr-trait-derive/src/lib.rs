//! A crate holding derive macros for the repr-trait crate.
//!
//! In general, prefer to use repr-trait instead of this.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttrStyle, Attribute, DeriveInput, Ident, Path};

macro_rules! repr_derive {
    ($tr:ident : $fn:ident($inner:expr) ) => {
        #[proc_macro_derive($tr)]
        pub fn $fn(input: TokenStream) -> TokenStream {
            let input = parse_macro_input!(input as DeriveInput);

            let ident = input.ident;

            if has_repr(&input.attrs, $inner) {
                quote!(
                    unsafe impl $tr for #ident {}
                ).into()
            } else {
                panic!("Can't derive {} on a struct without #[repr({})]", stringify!($tr), $inner);
            }
        }
    }
}

repr_derive!(Packed: repr_packed("packed"));
repr_derive!(Transparent: repr_transparent("transparent"));
repr_derive!(C: repr_c("C"));

#[proc_macro_derive(PrimitiveRepr)]
pub fn primitive_repr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data_enum = match &input.data {
        syn::Data::Struct(_) => panic!("Can't derive PrimitiveRepr on a struct"),
        syn::Data::Enum(data_enum) => data_enum,
        syn::Data::Union(_) => panic!("Can't derive PrimitiveRepr on an union"),
    };

    if data_enum.variants.is_empty() {
        panic!("Can't derive PrimitiveRepr on a zero variant enum");
    }

    if let Some(type_name) = find_repr_type(&input.attrs) {
        let ident = input.ident;
        let repr_ident = Ident::new(&type_name, ident.span());
        quote!(
            unsafe impl PrimitiveRepr for #ident {
                type Type = #repr_ident;
            }
        )
        .into()
    } else {
        panic!("Can't derive PrimitiveRepr on a struct without repr(u*) or repr(i*)");
    }
}

fn find_repr_type(attributes: &[Attribute]) -> Option<String> {
    for attr in attributes {
        // If the style isn't outer, reject it
        if !matches!(attr.style, AttrStyle::Outer) {
            continue;
        }
        // If the path doesn't match, reject it
        if let Path {
            leading_colon: None,
            ref segments,
        } = attr.path
        {
            // If there's more than one, reject it
            if segments.len() != 1 {
                continue;
            }

            let seg = segments.first().unwrap();

            // If there are arguments, reject it
            if !seg.arguments.is_empty() {
                continue;
            }

            // If the ident isn't "repr", reject it
            if seg.ident != "repr" {
                continue;
            }
        } else {
            // If we don't match, reject if
            continue;
        }

        let mut repr_type_name = format!("{}", attr.tokens);

        // Ensure repr is (u*) or (i*) and return what's inside.
        if (repr_type_name.starts_with("(u") || repr_type_name.starts_with("(i"))
            && repr_type_name.ends_with(')')
        {
            repr_type_name = repr_type_name[1..repr_type_name.len() - 1].to_string();
            return Some(repr_type_name);
        }
    }
    None
}

fn has_repr(attrs: &[Attribute], repr: &str) -> bool {
    for attr in attrs {
        // If the style isn't outer, reject it
        if !matches!(attr.style, AttrStyle::Outer) {
            continue;
        }

        // If the path doesn't match, reject it
        if let Path {
            leading_colon: None,
            ref segments,
        } = attr.path
        {
            // If there's more than one, reject it
            if segments.len() != 1 {
                continue;
            }

            let seg = segments.first().unwrap();

            // If there are arguments, reject it
            if !seg.arguments.is_empty() {
                continue;
            }

            // If the ident isn't "repr", reject it
            if seg.ident != "repr" {
                continue;
            }
        } else {
            // If we don't match, reject if
            continue;
        }

        // If it doesn't match, reject it

        if format!("{}", attr.tokens) != format!("({})", repr) {
            continue;
        }

        return true;
    }

    false
}
