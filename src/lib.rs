#![no_std]
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

#[macro_export]
macro_rules! bitfield {
    ($(#[$attribute:meta])* pub struct $($rest:tt)*) => {
        bitfield!($(#[$attribute])* (pub) struct $($rest)*);
    };
    ($(#[$attribute:meta])* struct $($rest:tt)*) => {
        bitfield!($(#[$attribute])* () struct $($rest)*);
    };
    ($(#[$attribute:meta])* ($($vis:tt)* )struct $name:ident([$t:ty]); $($rest:tt)*) => {
        bitfield_struct!($(#[$attribute])* $($vis)* struct $name([$t]));

        impl<T: AsMut<[$t]> + AsRef<[$t]>> $name<T> {
            bitfield_fields!{$($rest)*}
        }
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident(MSB0 [$t:ty]); $($rest:tt)*) => {
        bitfield_struct!($(#[$attribute])* $($vis)* struct $name(MSB0 [$t]));

        impl<T: AsMut<[$t]> + AsRef<[$t]>> $name<T> {
            bitfield_fields!{$($rest)*}
        }
    };
    ($(#[$attribute:meta])* ($($vis:tt)*) struct $name:ident($t:ty); $($rest:tt)*) => {
        bitfield_struct!($(#[$attribute])* $($vis)* struct $name($t));

        impl $name {
            bitfield_fields!{$t; $($rest)*}
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
