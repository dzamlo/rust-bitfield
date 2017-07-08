#[macro_use]
extern crate simple_bitfield;

simple_bitfield! {
    #[derive(Copy, Clone)]
    /// documentation comments also work!
    struct FooBar(u32);
    foo1, set_foo1: 0, 0;
    u8;
    foo2, set_foo2: 31, 31;
    foo3, set_foo3: 3, 0;
    u16, foo4, set_foo4: 31, 28;
    foo5, set_foo5: 0, 0, 32;
    u32;
    foo6, set_foo6: 5, 3, 3;
    getter_only, _: 3, 1;
    _, setter_only: 4, 2;
    getter_only_array, _: 5, 3, 3;
    _, setter_only_array: 6, 4, 3;
    all_bits, set_all_bits: 31, 0;
    single_bit, set_single_bit: 3;
}

#[test]
fn test_single_bit() {
    let mut fb = FooBar(0);

    fb.set_foo1(1);
    assert_eq!(0x1, fb.0);
    assert_eq!(0x1, fb.foo1());
    assert_eq!(0x0, fb.foo2());
    assert_eq!(false, fb.single_bit());

    fb.set_foo2(1);
    assert_eq!(0x80000001, fb.0);
    assert_eq!(0x1, fb.foo1());
    assert_eq!(0x1, fb.foo2());
    assert_eq!(false, fb.single_bit());

    fb.set_foo1(0);
    assert_eq!(0x80000000, fb.0);
    assert_eq!(0x0, fb.foo1());
    assert_eq!(0x1, fb.foo2());
    assert_eq!(false, fb.single_bit());

    fb.set_single_bit(true);
    assert_eq!(0x80000008, fb.0);
    assert_eq!(0x0, fb.foo1());
    assert_eq!(0x1, fb.foo2());
    assert_eq!(true, fb.single_bit());
}

#[test]
fn test_single_bit_plus_garbage() {
    let mut fb = FooBar(0);

    fb.set_foo1(0b10);
    assert_eq!(0x0, fb.0);
    assert_eq!(0x0, fb.foo1());
    assert_eq!(0x0, fb.foo2());

    fb.set_foo1(0b11);
    assert_eq!(0x1, fb.0);
    assert_eq!(0x1, fb.foo1());
    assert_eq!(0x0, fb.foo2());

}

#[test]
fn test_multiple_bit() {
    let mut fb = FooBar(0);

    fb.set_foo3(0x0F);
    assert_eq!(0xF, fb.0);
    assert_eq!(0xF, fb.foo3());
    assert_eq!(0x0, fb.foo4());

    fb.set_foo4(0x0F);
    assert_eq!(0xF000000F, fb.0);
    assert_eq!(0xF, fb.foo3());
    assert_eq!(0xF, fb.foo4());

    fb.set_foo3(0);
    assert_eq!(0xF0000000, fb.0);
    assert_eq!(0x0, fb.foo3());
    assert_eq!(0xF, fb.foo4());

    fb.set_foo3(0xA);
    assert_eq!(0xF000000A, fb.0);
    assert_eq!(0xA, fb.foo3());
    assert_eq!(0xF, fb.foo4());
}

#[test]
fn test_getter_setter_only() {
    let mut fb = FooBar(0);
    fb.setter_only(0x7);
    assert_eq!(0x1C, fb.0);
    assert_eq!(0x6, fb.getter_only());
}

#[test]
fn test_array_field1() {
    let mut fb = FooBar(0);

    fb.set_foo5(0, 1);
    assert_eq!(0x1, fb.0);
    assert_eq!(1, fb.foo5(0));

    fb.set_foo5(0, 0);
    assert_eq!(0x0, fb.0);
    assert_eq!(0, fb.foo5(0));

    fb.set_foo5(0, 1);
    fb.set_foo5(6, 1);
    fb.set_foo5(31, 1);
    assert_eq!(0x80000041, fb.0);
    assert_eq!(1, fb.foo5(0));
    assert_eq!(1, fb.foo5(6));
    assert_eq!(1, fb.foo5(31));
    assert_eq!(0, fb.foo5(1));
    assert_eq!(0, fb.foo5(5));
    assert_eq!(0, fb.foo5(7));
    assert_eq!(0, fb.foo5(30));
}

#[test]
fn test_array_field2() {
    let mut fb = FooBar(0);

    fb.set_foo6(0, 1);
    assert_eq!(0x8, fb.0);
    assert_eq!(1, fb.foo6(0));
    assert_eq!(0, fb.foo6(1));
    assert_eq!(0, fb.foo6(2));

    fb.set_foo6(0, 7);
    assert_eq!(0x38, fb.0);
    assert_eq!(7, fb.foo6(0));
    assert_eq!(0, fb.foo6(1));
    assert_eq!(0, fb.foo6(2));

    fb.set_foo6(2, 7);
    assert_eq!(0xE38, fb.0);
    assert_eq!(7, fb.foo6(0));
    assert_eq!(0, fb.foo6(1));
    assert_eq!(7, fb.foo6(2));

    fb.set_foo6(0, 0);
    assert_eq!(0xE00, fb.0);
    assert_eq!(0, fb.foo6(0));
    assert_eq!(0, fb.foo6(1));
    assert_eq!(7, fb.foo6(2));
}

#[allow(unknown_lints)]
#[allow(identity_op)]
#[test]
fn test_setter_only_array() {
    let mut fb = FooBar(0);

    fb.setter_only_array(0, 0);
    assert_eq!(0x0, fb.0);

    fb.setter_only_array(0, 0b111);
    assert_eq!(0b111 << (4 + 0 * 2), fb.0);

    fb.setter_only_array(0, 0);
    fb.setter_only_array(1, 0b111);
    assert_eq!(0b111 << (4 + 1 * 3), fb.0);

    fb.setter_only_array(1, 0);
    fb.setter_only_array(2, 0b111);
    assert_eq!(0b111 << (4 + 2 * 3), fb.0);
}

#[test]
fn test_getter_only_array() {
    let mut fb = FooBar(0);

    assert_eq!(0, fb.getter_only_array(0));
    assert_eq!(0, fb.getter_only_array(1));
    assert_eq!(0, fb.getter_only_array(2));

    fb.0 = !(0x1FF << 3);
    assert_eq!(0, fb.getter_only_array(0));
    assert_eq!(0, fb.getter_only_array(1));
    assert_eq!(0, fb.getter_only_array(2));

    fb.0 = 0xF << 3;
    assert_eq!(0b111, fb.getter_only_array(0));
    assert_eq!(0b001, fb.getter_only_array(1));
    assert_eq!(0, fb.getter_only_array(2));

    fb.0 = 0xF << 6;
    assert_eq!(0, fb.getter_only_array(0));
    assert_eq!(0b111, fb.getter_only_array(1));
    assert_eq!(0b001, fb.getter_only_array(2));

    fb.0 = 0xF << 8;
    assert_eq!(0, fb.getter_only_array(0));
    assert_eq!(0b100, fb.getter_only_array(1));
    assert_eq!(0b111, fb.getter_only_array(2));

    fb.0 = 0b101_010_110 << 3;
    assert_eq!(0b110, fb.getter_only_array(0));
    assert_eq!(0b010, fb.getter_only_array(1));
    assert_eq!(0b101, fb.getter_only_array(2));
}

#[test]
fn test_field_type() {
    let fb = FooBar(0);
    let _: u32 = fb.foo1();
    let _: u8 = fb.foo2();
    let _: u8 = fb.foo3();
    let _: u16 = fb.foo4();
    let _: u8 = fb.foo5(0);
    let _: u32 = fb.foo6(0);
}

#[test]
fn test_all_bits() {
    let mut fb = FooBar(0);

    assert_eq!(0, fb.all_bits());

    fb.set_all_bits(!0u32);
    assert_eq!(!0u32, fb.0);
    assert_eq!(!0u32, fb.all_bits());

    fb.0 = 0x80000001;
    assert_eq!(0x80000001, fb.all_bits());
}

#[test]
fn test_is_copy() {
    let a = FooBar(0);
    let _b = a;
    let _c = a;
}

simple_bitfield! {
    struct ArrayBitfield([u8]);
    foo1, set_foo1: 0, 0;
    foo2, set_foo2: 7, 0;
    foo3, set_foo3: 8, 1;
    foo4, set_foo4: 19, 4;
}

#[test]
fn test_arraybitfield() {
    let mut ab = ArrayBitfield([0; 3]);

    assert_eq!(0, ab.foo1());
    assert_eq!(0, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(1);
    assert_eq!([1, 0, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(1, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(0);
    ab.set_foo2(0xFF);
    assert_eq!([0xFF, 0, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(0xFF, ab.foo2());
    assert_eq!(0x7F, ab.foo3());
    assert_eq!(0x0F, ab.foo4());

    ab.set_foo2(0);
    ab.set_foo3(0xFF);
    assert_eq!([0xFE, 0x1, 0], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0xFE, ab.foo2());
    assert_eq!(0xFF, ab.foo3());
    assert_eq!(0x1F, ab.foo4());

    ab.set_foo3(0);
    ab.set_foo4(0xFFFF);
    assert_eq!([0xF0, 0xFF, 0x0F], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0xF0, ab.foo2());
    assert_eq!(0xF8, ab.foo3());
    assert_eq!(0xFFFF, ab.foo4());
}

simple_bitfield! {
    struct ArrayBitfield2([u16]);
    foo1, set_foo1: 0, 0;
    foo2, set_foo2: 7, 0;
    foo3, set_foo3: 8, 1;
    foo4, set_foo4: 20, 4;
}

#[test]
fn test_arraybitfield2() {
    let mut ab = ArrayBitfield2([0; 2]);

    assert_eq!(0, ab.foo1());
    assert_eq!(0, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(1);
    assert_eq!([1, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(1, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(0);
    ab.set_foo2(0xFF);
    assert_eq!([0xFF, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(0xFF, ab.foo2());
    assert_eq!(0x7F, ab.foo3());
    assert_eq!(0x0F, ab.foo4());

    ab.set_foo2(0);
    ab.set_foo3(0xFF);
    assert_eq!([0x1FE, 0x0], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0xFE, ab.foo2());
    assert_eq!(0xFF, ab.foo3());
    assert_eq!(0x1F, ab.foo4());

    ab.set_foo3(0);
    ab.set_foo4(0xFFFF);
    assert_eq!([0xFFF0, 0xF], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0xF0, ab.foo2());
    assert_eq!(0xF8, ab.foo3());
    assert_eq!(0xFFFF, ab.foo4());
}

simple_bitfield! {
    struct ArrayBitfieldMsb0(MSB0 [u8]);
    foo1, set_foo1: 0, 0;
    foo2, set_foo2: 7, 0;
    foo3, set_foo3: 8, 1;
    foo4, set_foo4: 19, 4;
}

#[test]
fn test_arraybitfield_msb0() {
    let mut ab = ArrayBitfieldMsb0([0; 3]);

    assert_eq!(0, ab.foo1());
    assert_eq!(0, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(1);
    assert_eq!([0b1000_0000, 0, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(0b1000_0000, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(0);
    ab.set_foo2(0xFF);
    assert_eq!([0b1111_1111, 0, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(0b1111_1111, ab.foo2());
    assert_eq!(0b1111_1110, ab.foo3());
    assert_eq!(0b1111_0000_0000_0000, ab.foo4());

    ab.set_foo2(0);
    ab.set_foo3(0xFF);
    assert_eq!([0b01111111, 0b10000000, 0], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0b01111111, ab.foo2());
    assert_eq!(0xFF, ab.foo3());
    assert_eq!(0b1111_1000_0000_0000, ab.foo4());

    ab.set_foo3(0);
    ab.set_foo4(0xFFFF);
    assert_eq!([0x0F, 0xFF, 0xF0], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0x0F, ab.foo2());
    assert_eq!(0b0001_1111, ab.foo3());
    assert_eq!(0xFFFF, ab.foo4());
}

mod some_module {
    simple_bitfield! {
        pub struct PubBitFieldInAModule(u8);
    }
}

#[test]
fn struct_can_be_public() {
    let _ = some_module::PubBitFieldInAModule(0);
}
