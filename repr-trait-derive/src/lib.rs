use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttrStyle, Attribute, DeriveInput, Path};

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

fn has_repr(attrs: &[Attribute], repr: &str) -> bool {
    for attr in attrs {
        // If the style isn't outer, reject it
        if attr.style != AttrStyle::Outer {
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
            if seg.ident.to_string() != "repr" {
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
