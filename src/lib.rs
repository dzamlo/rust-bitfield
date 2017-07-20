#![no_std]
#![deny(missing_docs,unused_extern_crates, unused_import_braces, unused_qualifications )]

//!  This crate provides macros to generate bitfield-like struct.
//!
//!  See the documentation of the macros for how to use them.
//!
//!  Examples and tests are also a great way to understand how to use these macros.

/// Declares the fields of struct.
///
/// This macro will generate the methods to access the fields of a bitfield. It must be called
/// from an `impl` block for a type that implements the `BitRange` and/or the `Bit` traits
/// (which traits are required depending on what type of fields are used).
///
/// The syntax of this macro is composed of declarations ended by semicolons. There are two types
/// of delcarations: default type, and fields.
///
/// A default type is just a type followed by a semicolon. This will affect all the following field
/// declarations.
///
/// A field declaration is composed of the following:
///
/// * Optional attributes (`#[...]`), documentation comments (`///`) are attributes;
/// * An optional pub keyword to make the methods public
/// * An optional type
/// * The getter and setter idents, separated by a comma
/// * A colon
/// * One to three expressions of type `usize`
///
/// The attributes and pub will be applied to the two methods generated.
///
/// The getter and setter idents can be `_` to not generate one of the two. For example, if the
/// setter is `_`, the field will be read-only.
///
/// The expressions at the end are the bit positions. Their meaning depends on the number of
/// expressions:
///
///  * One expression: the field is a single bit. The type is ignored and `bool` is used. The trait
///    `Bit` is used.
///  * Two expressions: `msb, lsb`, the field is composed of the bits from `msb` to `lsb`, included.
///  * Three expressions: `msb, lsb, count`, the field is an array. The first element is composed of
///    the bits from `msb` to `lsb`. The following elements are consecutive bits range of the same
///    size.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate bitfield;
/// # fn main() {}
/// # bitfield_struct!{struct FooBar(u64)}
/// # impl FooBar {
/// bitfield_fields!{
///     // The default type will be `u64
///     u64;
///     // filed1 is read-write, public, the methods are inline
///     #[inline]
///     pub field1, set_field1: 10, 0;
///     // `field2` is  read-only, private, and of type bool.
///     field2, _ : 0;
/// }
/// # }
/// ```
#[macro_export]
macro_rules! bitfield_fields {
    (@field $(#[$attribute:meta])* ($($vis:tt)*) $t:ty, _, $setter:ident: $msb:expr, $lsb:expr,
     $count:expr) => {
        $(#[$attribute])*
        #[allow(unknown_lints)]
        #[allow(eq_op)]
        $($vis)* fn $setter(&mut self, index: usize, value: $t) {
            use $crate::BitRange;
            debug_assert!(index < $count);
            let width = $msb - $lsb + 1;
            let lsb = $lsb + index*width;
            let msb = lsb + width - 1;
            self.set_bit_range(msb, lsb, value);
        }
    };
    (@field $(#[$attribute:meta])* ($($vis:tt)*) $t:ty, _, $setter:ident: $msb:expr, $lsb:expr) => {
        $(#[$attribute])*
        $($vis)* fn $setter(&mut self, value: $t) {
            use $crate::BitRange;
            self.set_bit_range($msb, $lsb, value);
        }
    };
    (@field $(#[$attribute:meta])* ($($vis:tt)*) $t:ty, _, $setter:ident: $bit:expr) => {
        $(#[$attribute])*
        $($vis)* fn $setter(&mut self, value: bool) {
            use $crate::Bit;
            self.set_bit($bit, value);
        }
    };
    (@field $(#[$attribute:meta])* ($($vis:tt)*) $t:ty, $getter:ident, _: $msb:expr, $lsb:expr,
     $count:expr) => {
        $(#[$attribute])*
        #[allow(unknown_lints)]
        #[allow(eq_op)]
        $($vis)* fn $getter(&self, index: usize) -> $t {
            use $crate::BitRange;
            debug_assert!(index < $count);
            let width = $msb - $lsb + 1;
            let lsb = $lsb + index*width;
            let msb = lsb + width - 1;
            self.bit_range(msb, lsb)
        }
    };
    (@field $(#[$attribute:meta])* ($($vis:tt)*) $t:ty, $getter:ident, _: $msb:expr, $lsb:expr) => {
        $(#[$attribute])*
        $($vis)* fn $getter(&self) -> $t {
            use $crate::BitRange;
            self.bit_range($msb, $lsb)
        }
    };
    (@field $(#[$attribute:meta])* ($($vis:tt)*) $t:ty, $getter:ident, _: $bit:expr) => {
        $(#[$attribute])*
        $($vis)* fn $getter(&self) -> bool {
            use $crate::Bit;
            self.bit($bit)
        }
    };
    (@field $(#[$attribute:meta])* ($($vis:tt)*) $t:ty, $getter:ident, $setter:ident:
     $($exprs:expr),*) => {
        bitfield_fields!(@field $(#[$attribute])* ($($vis)*) $t, $getter, _: $($exprs),*);
        bitfield_fields!(@field $(#[$attribute])* ($($vis)*) $t, _, $setter: $($exprs),*);
    };
    ($t:ty;) => {};
    ($default_ty:ty; pub $($rest:tt)*) => {
        bitfield_fields!{$default_ty; () pub $($rest)*}
    };
    ($default_ty:ty; #[$attribute:meta] $($rest:tt)*) => {
        bitfield_fields!{$default_ty; (#[$attribute]) $($rest)*}
    };
    ($default_ty:ty; ($(#[$attributes:meta])*) #[$attribute:meta] $($rest:tt)*) => {
        bitfield_fields!{$default_ty; ($(#[$attributes])* #[$attribute]) $($rest)*}
    };
    ($default_ty:ty; ($(#[$attribute:meta])*) pub $t:ty, $getter:tt, $setter:tt:  $($exprs:expr),*;
     $($rest:tt)*) => {
        bitfield_fields!{@field $(#[$attribute])* (pub) $t, $getter, $setter: $($exprs),*}
        bitfield_fields!{$default_ty; $($rest)*}
    };
    ($default_ty:ty; ($(#[$attribute:meta])*) pub $getter:tt, $setter:tt:  $($exprs:expr),*;
     $($rest:tt)*) => {
        bitfield_fields!{@field $(#[$attribute])* (pub) $default_ty, $getter, $setter:
                                $($exprs),*}
        bitfield_fields!{$default_ty; $($rest)*}
    };
    ($default_ty:ty; ($(#[$attribute:meta])*) $t:ty, $getter:tt, $setter:tt:  $($exprs:expr),*;
     $($rest:tt)*) => {
        bitfield_fields!{@field $(#[$attribute])* () $t, $getter, $setter: $($exprs),*}
        bitfield_fields!{$default_ty; $($rest)*}
    };
    ($default_ty:ty; ($(#[$attribute:meta])*) $getter:tt, $setter:tt:  $($exprs:expr),*;
     $($rest:tt)*) => {
        bitfield_fields!{@field $(#[$attribute])* () $default_ty, $getter, $setter:
                                $($exprs),*}
        bitfield_fields!{$default_ty; $($rest)*}
    };
    ($previous_default_ty:ty; $default_ty:ty; $($rest:tt)*) => {
        bitfield_fields!{$default_ty; $($rest)*}
    };
    ($default_ty:ty; $($rest:tt)*) => {
        bitfield_fields!{$default_ty; () $($rest)*}
    };
    ($($rest:tt)*) => {
        bitfield_fields!{SET_A_DEFAULT_TYPE_OR_SPECIFY_THE_TYPE_FOR_EACH_FIELDS; $($rest)*}
    }
}

/// Generates a `fmt::Debug` implementation.
///
/// This macros must be called from a `impl Debug for ...` block. It will generate the `fmt` method.
///
/// In most of the case, you will not directly call this macros, but use `bitfield`.
///
/// The syntax is `struct TheNameOfTheStruct` folowowed by the syntax of `bitfield_fields`.
///
/// The write-only fields are ignored.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate bitfield;
/// bitfield_struct!{struct FooBar(u32)}
/// impl FooBar{
///     bitfield_fields!{
///        u32;
///        field1, _: 7, 0;
///        field2, _: 31, 24;
///     }
/// }
///
/// impl std::fmt::Debug for FooBar {
///     bitfield_debug!{
///        struct FooBar;
///        field1, _: 7, 0;
///        field2, _: 31, 24;
///     }
/// }
///
/// fn main() {
///     let foobar = FooBar(0x11223344);
///     println!("{:?}", foobar);

/// }
/// ```
#[macro_export]
macro_rules! bitfield_debug {
    (struct $name:ident; $($rest:tt)*) => {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            let mut debug_struct = f.debug_struct(stringify!($name));
            debug_struct.field(".0", &self.0);
            bitfield_debug!{debug_struct, self, $($rest)*}
            debug_struct.finish()
        }
    };
    ($debug_struct:ident, $self:ident, #[$attribute:meta] $($rest:tt)*) => {
        bitfield_debug!{$debug_struct, $self, $($rest)*}
    };
    ($debug_struct:ident, $self:ident, pub $($rest:tt)*) => {
        bitfield_debug!{$debug_struct, $self, $($rest)*}
    };
    ($debug_struct:ident, $self:ident, _, $setter:tt: $($exprs:expr),*; $($rest:tt)*) => {
        bitfield_debug!{$debug_struct, $self, $($rest)*}
    };
    ($debug_struct:ident, $self:ident, $type:ty; $($rest:tt)*) => {
        bitfield_debug!{$debug_struct, $self, $($rest)*}
    };
    ($debug_struct:ident, $self:ident, $getter:ident, $setter:tt: $msb:expr, $lsb:expr, $count:expr;
     $($rest:tt)*) => {
        let mut array = [$self.$getter(0); $count];
        for (i, e) in (&mut array).into_iter().enumerate() {
            *e = $self.$getter(i);
        }
        $debug_struct.field(stringify!($getter), &array);
        bitfield_debug!{$debug_struct, $self, $($rest)*}
    };
    ($debug_struct:ident, $self:ident, $getter:ident, $setter:tt: $($exprs:expr),*; $($rest:tt)*)
        => {
        $debug_struct.field(stringify!($getter), &$self.$getter());
        bitfield_debug!{$debug_struct, $self, $($rest)*}
    };
    ($debug_struct:ident, $self:ident, $type:ty, $($rest:tt)*) => {
        bitfield_debug!{$debug_struct, $self, $($rest)*}
    };
    ($debug_struct:ident, $self:ident, ) => {};
}

/// Declares a struct that implements `BitRange`,
///
/// This macro will generate a tuple struct (or "newtype") that implements the `BitRange` trait and
/// by extension the `Bit` trait.
///
/// The syntax is more or less the same as declaring a "newtype", including the attributes,
/// documentation comments and pub keyword.
///
/// The difference with a normal "newtype" is the type in parentheses. If the type is `[t]` (where
/// `t` is any of the unsigned integer type), the "newtype" will be generic and implement
/// `BitRange` for `T: AsMut<[t]> + AsRef<[t]>` (for example a slice, an array or a `Vec`). You can
/// also use `MSB0 [t]`. The difference will be the positions of the bit. You can use the
/// `bits_positions` example to see where each bits is. If the type is neither of this two, the
/// "newtype" will wrap a value of the specified type and implements `BitRange` the same ways as
/// the wrapped type.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate bitfield;
/// # fn main() {}
/// bitfield_struct!{struct BitField1(u32)}
///
/// bitfield_struct!{
///     /// The documentation for this type
///     #[derive(Copy, Clone)]
///     pub struct BitField2(u64)
/// }
///
/// bitfield_struct!{struct BitField3([u8])}
///
/// bitfield_struct!{struct BitField4(MSB0 [u8])}
/// ```
///
#[macro_export]
macro_rules! bitfield_struct {
    (@impl_bitrange_slice $name:ident, $slice_ty:ty, $bitrange_ty:ty) => {
        impl<T: AsMut<[$slice_ty]> + AsRef<[$slice_ty]>> $crate::BitRange<$bitrange_ty>
            for $name<T> {
                fn bit_range(&self, msb: usize, lsb: usize) -> $bitrange_ty {
                    let bit_len = $crate::size_of::<$slice_ty>()*8;
                    let mut value = 0;
                    for i in (lsb..msb+1).rev() {
                        value <<= 1;
                        value |= ((self.0.as_ref()[i/bit_len] >> (i%bit_len)) & 1) as $bitrange_ty;
                    }
                    value
                }

                fn set_bit_range(&mut self, msb: usize, lsb: usize, value: $bitrange_ty) {
                    let bit_len = $crate::size_of::<$slice_ty>()*8;
                    let mut value = value;
                    for i in lsb..msb+1 {
                        self.0.as_mut()[i/bit_len] &= !(1 << (i%bit_len));
                        self.0.as_mut()[i/bit_len] |= (value & 1) as $slice_ty << (i%bit_len);
                        value >>= 1;
                    }
                }
            }
    };
    (@impl_bitrange_slice_msb0 $name:ident, $slice_ty:ty, $bitrange_ty:ty) => {
        impl<T: AsMut<[$slice_ty]> + AsRef<[$slice_ty]>> $crate::BitRange<$bitrange_ty>
            for $name<T> {
            fn bit_range(&self, msb: usize, lsb: usize) -> $bitrange_ty {
                let bit_len = $crate::size_of::<$slice_ty>()*8;
                let mut value = 0;
                for i in lsb..msb+1 {
                    value <<= 1;
                    value |= ((self.0.as_ref()[i/bit_len] >> (bit_len - i%bit_len - 1)) & 1)
                        as $bitrange_ty;
                }
                value
            }

            fn set_bit_range(&mut self, msb: usize, lsb: usize, value: $bitrange_ty) {
                let bit_len = $crate::size_of::<$slice_ty>()*8;
                let mut value = value;
                for i in (lsb..msb+1).rev() {
                    self.0.as_mut()[i/bit_len] &= !(1 << (bit_len - i%bit_len - 1));
                    self.0.as_mut()[i/bit_len] |= (value & 1) as $slice_ty
                        << (bit_len - i%bit_len - 1);
                    value >>= 1;
                }
            }
        }
    };
    ($(#[$attribute:meta])* struct $name:ident($($args:tt)*)) => {
        bitfield_struct!($(#[$attribute])* () struct $name($($args)*));
    };
    ($(#[$attribute:meta])* pub struct $name:ident($($args:tt)*))=> {
        bitfield_struct!($(#[$attribute])* (pub) struct $name($($args)*));
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident([$t:ty])) => {
        $(#[$attribute])*
        $($vis)* struct $name<T>(pub T);

        bitfield_struct!(@impl_bitrange_slice $name, $t, u8);
        bitfield_struct!(@impl_bitrange_slice $name, $t, u16);
        bitfield_struct!(@impl_bitrange_slice $name, $t, u32);
        bitfield_struct!(@impl_bitrange_slice $name, $t, u64);
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident(MSB0 [$t:ty])) => {
        $(#[$attribute])*
        $($vis)* struct $name<T>(pub T);

        bitfield_struct!(@impl_bitrange_slice_msb0 $name, $t, u8);
        bitfield_struct!(@impl_bitrange_slice_msb0 $name, $t, u16);
        bitfield_struct!(@impl_bitrange_slice_msb0 $name, $t, u32);
        bitfield_struct!(@impl_bitrange_slice_msb0 $name, $t, u64);
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident($t:ty)) => {
        $(#[$attribute])*
        $($vis)* struct $name(pub $t);

        impl<T> $crate::BitRange<T> for $name where $t: $crate::BitRange<T> {
            fn bit_range(&self, msb: usize, lsb: usize) -> T {
                self.0.bit_range(msb, lsb)
            }
            fn set_bit_range(&mut self, msb: usize, lsb: usize, value: T) {
                self.0.set_bit_range(msb, lsb, value);
            }
        }
    };
}

/// Combines `bitfield_struct` and `bitfield_fields`.
///
/// The syntax of this macro is the syntax of `bitfield_struct`, a semicolon, and then the syntax
/// of `bitfield_fields`.
///
/// If you put `impl Debug;` after the first semicolon, an implementation of `fmt::Debug` will be
/// generated with the `bitfield_debug` macro.
///
/// The difference with calling those macros separately is that `bitfield_fields` is called
/// from an appropriate `impl` block. If you use the non-slice form of `bitfield_struct`, the
/// default type for `bitfield_fields` will be set to the wrapped fields.
///
/// See the documentation of these macros more information on their respective syntax.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate bitfield;
/// # fn main() {}
/// bitfield!{
///   pub struct BitField1(u16);
///   impl Debug;
///   // The fields default to u16
///   field1, set_field1: 10, 0;
///   pub field2, _ : 12, 3;
/// }
/// ```
#[macro_export]
macro_rules! bitfield {
    ($(#[$attribute:meta])* pub struct $($rest:tt)*) => {
        bitfield!($(#[$attribute])* (pub) struct $($rest)*);
    };
    ($(#[$attribute:meta])* struct $($rest:tt)*) => {
        bitfield!($(#[$attribute])* () struct $($rest)*);
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident([$t:ty]); impl Debug; $($rest:tt)*)
        => {
        impl<T: AsMut<[$t]> + AsRef<[$t]> + $crate::fmt::Debug> $crate::fmt::Debug for $name<T> {
            bitfield_debug!{struct $name; $($rest)*}
        }

        bitfield!{$(#[$attribute])* ($($vis)*) struct $name([$t]); $($rest)*}
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident([$t:ty]); $($rest:tt)*) => {
        bitfield_struct!($(#[$attribute])* $($vis)* struct $name([$t]));

        impl<T: AsMut<[$t]> + AsRef<[$t]>> $name<T> {
            bitfield_fields!{$($rest)*}
        }
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident(MSB0 [$t:ty]); impl Debug;
     $($rest:tt)*) => {
        impl<T: AsMut<[$t]> + AsRef<[$t]> + $crate::fmt::Debug> $crate::fmt::Debug for $name<T> {
            bitfield_debug!{struct $name; $($rest)*}
        }

        bitfield!{$(#[$attribute])* ($($vis)*) struct $name(MSB0 [$t]); $($rest)*}
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident(MSB0 [$t:ty]); $($rest:tt)*) => {
        bitfield_struct!($(#[$attribute])* $($vis)* struct $name(MSB0 [$t]));

        impl<T: AsMut<[$t]> + AsRef<[$t]>> $name<T> {
            bitfield_fields!{$($rest)*}
        }
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident($t:ty); impl Debug; $($rest:tt)*) => {
        impl $crate::fmt::Debug for $name {
            bitfield_debug!{struct $name; $($rest)*}
        }

        bitfield!{$(#[$attribute])* ($($vis)*) struct $name($t); $($rest)*}
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident($t:ty); $($rest:tt)*) => {
        bitfield_struct!($(#[$attribute])* $($vis)* struct $name($t));

        impl $name {
            bitfield_fields!{$t; $($rest)*}
         }
    };
}


#[doc(hidden)]
pub use core::fmt;
#[doc(hidden)]
pub use core::mem::size_of;

/// A trait to get or set ranges of bits.
pub trait BitRange<T> {
    /// Get a range of bits.
    fn bit_range(&self, msb: usize, lsb: usize) -> T;
    /// Set a range of bits.
    fn set_bit_range(&mut self, msb: usize, lsb: usize, value: T);
}

/// A trait to get or set a single bit.
///
/// This trait is implemented for all type that implement `BitRange<u8>`.
pub trait Bit {
    /// Get a single bit.
    fn bit(&self, bit: usize) -> bool;

    /// Set a single bit.
    fn set_bit(&mut self, bit: usize, value: bool);
}

impl<T: BitRange<u8>> Bit for T {
    fn bit(&self, bit: usize) -> bool {
        self.bit_range(bit, bit) != 0
    }
    fn set_bit(&mut self, bit: usize, value: bool) {
        self.set_bit_range(bit, bit, value as u8);
    }
}

macro_rules! impl_bitrange_for_u {
    ($t:ty, $bitrange_ty:ty) => {
        impl BitRange<$bitrange_ty> for $t {
            #[inline]
            fn bit_range(&self, msb: usize, lsb: usize) -> $bitrange_ty {
                let bit_len = size_of::<$t>()*8;
                ((*self << (bit_len - msb - 1)) >> (bit_len - msb - 1 + lsb)) as $bitrange_ty
            }

            #[inline]
            fn set_bit_range(&mut self, msb: usize, lsb: usize, value: $bitrange_ty) {
                let bit_len = size_of::<$t>()*8;
                let mask: $t = !(0 as $t)
                    << (bit_len - msb - 1)
                    >> (bit_len - msb - 1 + lsb)
                        << (lsb);
                *self &= !mask;
                *self |= (value as $t << lsb) & mask;
            }
        }
    }
}

impl_bitrange_for_u!{u8, u8}
impl_bitrange_for_u!{u16, u8}
impl_bitrange_for_u!{u16, u16}
impl_bitrange_for_u!{u32, u8}
impl_bitrange_for_u!{u32, u16}
impl_bitrange_for_u!{u32, u32}
impl_bitrange_for_u!{u64, u8}
impl_bitrange_for_u!{u64, u16}
impl_bitrange_for_u!{u64, u32}
impl_bitrange_for_u!{u64, u64}
