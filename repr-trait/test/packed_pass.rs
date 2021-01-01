use repr_trait::Packed;

#[derive(repr_trait::Packed)]
#[repr(packed)]
struct AmPacked();

fn accept_packed(_: impl Packed) { }

fn main() {
    accept_packed(AmPacked());
}
