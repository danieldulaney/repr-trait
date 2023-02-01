use repr_trait::{self, PrimitiveRepr};

#[derive(PrimitiveRepr)]
#[repr(i64)]
enum SimpleEnum {
    A,
    B,
    C,
}

#[derive(PrimitiveRepr)]
#[repr(usize)]
enum ComplexEnum {
    Unit,
    Tuple(u64) = 154,
    Struct { _a: bool },
}

fn discriminant_value<T>(value: &T) -> T::Type
where
    T: PrimitiveRepr,
    T::Type: Copy,
{
    *repr_trait::discriminant(value)
}

#[test]
fn can_access_discriminant_of_complex_enum() {
    assert_eq!(discriminant_value(&ComplexEnum::Unit), 0usize);
    assert_eq!(discriminant_value(&ComplexEnum::Tuple(42)), 154usize);
    assert_eq!(
        discriminant_value(&ComplexEnum::Struct { _a: true }),
        155usize
    );
}

#[test]
fn can_access_discriminant_of_simple_enum() {
    assert_eq!(discriminant_value(&SimpleEnum::A), 0i64);
    assert_eq!(discriminant_value(&SimpleEnum::B), 1i64);
    assert_eq!(discriminant_value(&SimpleEnum::C), 2i64);
}
