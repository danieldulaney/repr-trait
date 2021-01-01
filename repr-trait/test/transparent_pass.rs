use repr_trait::Transparent;

#[derive(repr_trait::Transparent)]
#[repr(transparent)]
struct AmTransparent(u32);

fn accept_transparent(_: impl Transparent) { }

fn main() {
    accept_transparent(AmTransparent(123));
}
