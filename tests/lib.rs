#[macro_use]
extern crate simple_bitfield;

use std::mem;

simple_bitfield! {
    FooBar, u32, // newtype name, underlying type
    foo1, set_foo1: 0, 0, // getter name, setter name: msb (included), lsb
    foo2, set_foo2: 31, 31,
    foo3, set_foo3: 3, 0,
    foo4, set_foo4: 31, 28,
    getter_only, _: 3, 1,
    _, setter_only: 4, 2,
}

#[test]
fn test_single_bit() {
   let mut fb = FooBar(0);

   fb.set_foo1(1);
   assert_eq!(0x1, fb.0);
   assert_eq!(0x1, fb.foo1());
   assert_eq!(0x0, fb.foo2());

   fb.set_foo2(1);
   assert_eq!(0x80000001, fb.0);
   assert_eq!(0x1, fb.foo1());
   assert_eq!(0x1, fb.foo2());

   fb.set_foo1(0);
   assert_eq!(0x80000000, fb.0);
   assert_eq!(0x0, fb.foo1());
   assert_eq!(0x1, fb.foo2());
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
