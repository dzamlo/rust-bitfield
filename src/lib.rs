#![no_std]

/// This macros defines a single field of a bitfiled.
///
/// It must be called from an impl and will generates methods. You should not
/// call this macro directly. Use `simple_bitfiled_fields` instead.
#[macro_export]
macro_rules! simple_bitfield_field {
    (($($vis:tt)*) $t:ty, _, $setter:ident: $msb:expr, $lsb:expr, $count:expr) => {
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
    (($($vis:tt)*) $t:ty, _, $setter:ident: $msb:expr, $lsb:expr) => {
        $($vis)* fn $setter(&mut self, value: $t) {
            use $crate::BitRange;
            self.set_bit_range($msb, $lsb, value);
        }
    };
    (($($vis:tt)*) $t:ty, _, $setter:ident: $bit:expr) => {
        $($vis)* fn $setter(&mut self, value: bool) {
            use $crate::Bit;
            self.set_bit($bit, value);
        }
    };
    (($($vis:tt)*) $t:ty, $getter:ident, _: $msb:expr, $lsb:expr, $count:expr) => {
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
    (($($vis:tt)*) $t:ty, $getter:ident, _: $msb:expr, $lsb:expr) => {
        $($vis)* fn $getter(&self) -> $t {
            use $crate::BitRange;
            self.bit_range($msb, $lsb)
        }
    };
    (($($vis:tt)*) $t:ty, $getter:ident, _: $bit:expr) => {
        $($vis)* fn $getter(&self) -> bool {
            use $crate::Bit;
            self.bit($bit)
        }
    };
    (($($vis:tt)*) $t:ty, $getter:ident, $setter:ident: $($exprs:expr),*) => {
        simple_bitfield_field!(($($vis)*) $t, $getter, _: $($exprs),*);
        simple_bitfield_field!(($($vis)*) $t, _, $setter: $($exprs),*);
    };
}

#[macro_export]
macro_rules! simple_bitfield_fields {
    ($t:ty;) => {};
    ($default_ty:ty; pub $t:ty, $getter:tt, $setter:tt:  $($exprs:expr),*; $($rest:tt)*) => {
        simple_bitfield_field!{(pub) $t, $getter, $setter: $($exprs),*}
        simple_bitfield_fields!{$default_ty; $($rest)*}
    };
    ($default_ty:ty; pub $getter:tt, $setter:tt:  $($exprs:expr),*; $($rest:tt)*) => {
        simple_bitfield_field!{(pub) $default_ty, $getter, $setter: $($exprs),*}
        simple_bitfield_fields!{$default_ty; $($rest)*}
    };
    ($default_ty:ty; $t:ty, $getter:tt, $setter:tt:  $($exprs:expr),*; $($rest:tt)*) => {
        simple_bitfield_field!{() $t, $getter, $setter: $($exprs),*}
        simple_bitfield_fields!{$default_ty; $($rest)*}
    };
    ($default_ty:ty; $getter:tt, $setter:tt:  $($exprs:expr),*; $($rest:tt)*) => {
        simple_bitfield_field!{() $default_ty, $getter, $setter: $($exprs),*}
        simple_bitfield_fields!{$default_ty; $($rest)*}
    };
    ($previous_default_ty:ty; $default_ty:ty; $($rest:tt)*) => {
        simple_bitfield_fields!{$default_ty; $($rest)*}
    };
}

#[macro_export]
macro_rules! simple_bitfield_struct {
    ($(#[$attribute:meta])* struct $name:ident($($args:tt)*)) => {
        simple_bitfield_struct!($(#[$attribute])* () struct $name($($args)*));
    };
    ($(#[$attribute:meta])* pub struct $name:ident($($args:tt)*))=> {
        simple_bitfield_struct!($(#[$attribute])* (pub) struct $name($($args)*));
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident([$t:ty])) => {
        $(#[$attribute])*
        $($vis)* struct $name<T>(pub T);

        impl_bitrange_slice!($name, $t, u8);
        impl_bitrange_slice!($name, $t, u16);
        impl_bitrange_slice!($name, $t, u32);
        impl_bitrange_slice!($name, $t, u64);
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident(MSB0 [$t:ty])) => {
        $(#[$attribute])*
        $($vis)* struct $name<T>(pub T);

        impl_bitrange_slice_msb0!($name, $t, u8);
        impl_bitrange_slice_msb0!($name, $t, u16);
        impl_bitrange_slice_msb0!($name, $t, u32);
        impl_bitrange_slice_msb0!($name, $t, u64);
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

#[macro_export]
macro_rules! simple_bitfield {
    ($(#[$attribute:meta])* pub struct $($rest:tt)*) => {
        simple_bitfield!($(#[$attribute])* (pub) struct $($rest)*);
    };
    ($(#[$attribute:meta])* struct $($rest:tt)*) => {
        simple_bitfield!($(#[$attribute])* () struct $($rest)*);
    };
    ($(#[$attribute:meta])* ($($vis:tt)* )struct $name:ident([$t:ty]); $($rest:tt)*) => {
        simple_bitfield_struct!($(#[$attribute])* $($vis)* struct $name([$t]));

        impl<T: AsMut<[$t]> + AsRef<[$t]>> $name<T> {
            simple_bitfield_fields!{u64; $($rest)*}
        }
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident(MSB0 [$t:ty]); $($rest:tt)*) => {
        simple_bitfield_struct!($(#[$attribute])* $($vis)* struct $name(MSB0 [$t]));

        impl<T: AsMut<[$t]> + AsRef<[$t]>> $name<T> {
            simple_bitfield_fields!{u64; $($rest)*}
        }
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident($t:ty); $($rest:tt)*) => {
        simple_bitfield_struct!($(#[$attribute])* $($vis)* struct $name($t));

        impl $name {
            simple_bitfield_fields!{$t; $($rest)*}
         }
    };
}

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

#[doc(hidden)]
#[macro_export]
macro_rules! impl_bitrange_slice {
    ($name:ident, $slice_ty:ty, $bitrange_ty:ty) => {
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
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_bitrange_slice_msb0 {
    ($name:ident, $slice_ty:ty, $bitrange_ty:ty) => {
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
    }
}
