mod fields {
    use bitfield::bitfield;

    bitfield! {
        pub struct Integer(pub u8);
        impl Debug;
        impl new;
        pub u8, value, set_value: 7, 0;
    }

    bitfield! {
        pub struct Slice(pub [u8]);
        impl Debug;
        pub u8, value, set_value: 7, 0;
    }

    bitfield! {
        pub struct Msb0(pub MSB0 [u8]);
        impl Debug;
        pub u8, value, set_value: 7, 0;
    }

    // Exercise visibility forwarding when impl declarations are reordered.
    bitfield! {
        pub struct CustomInteger(pub(crate) u8);
        impl Debug;
        no default BitRange;
    }

    bitfield! {
        pub struct CustomSlice(pub(super) [u8]);
        impl Debug;
        no default BitRange;
    }

    bitfield! {
        pub struct CustomMsb0(pub MSB0 [u8]);
        no default BitRange;
        impl Debug;
    }
}

#[test]
fn public_inner_fields_are_accessible_outside_the_defining_module() {
    let mut integer = fields::Integer(0);
    integer.set_value(42);
    assert_eq!(integer.0, 42);
    assert_eq!(fields::Integer::new(42).value(), 42);

    let mut slice = fields::Slice([0]);
    slice.set_value(42);
    assert_eq!(slice.0, [42]);
    assert_eq!(slice.value(), 42);

    let mut msb0 = fields::Msb0([0]);
    msb0.set_value(42);
    assert_eq!(msb0.0, [42]);
    assert_eq!(msb0.value(), 42);
}

#[test]
fn explicit_visibility_is_preserved_without_default_bitrange() {
    assert_eq!(fields::CustomInteger(42).0, 42);
    assert_eq!(fields::CustomSlice([42]).0, [42]);
    assert_eq!(fields::CustomMsb0([42]).0, [42]);
}
