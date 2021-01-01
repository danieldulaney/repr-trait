use repr_trait::C;

#[derive(repr_trait::C)]
#[repr(C)]
struct AmC();

fn accept_c(_: impl C) { }

fn main() {
    accept_c(AmC());
}
