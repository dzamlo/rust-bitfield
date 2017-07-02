#![no_std]

#[macro_export]
macro_rules! simple_bitfield_fields {
   ($t:ty,) => {};
   ($t:ty, _, $setter:ident: $msb:expr, $lsb:expr; $($rest:tt)*) => {
       pub fn $setter(&mut self, value: $t) {
           use $crate::BitRange;
           self.set_bit_range($msb, $lsb, value);
       }
       simple_bitfield_fields!{$t, $($rest)*}
   };
   ($t:ty, _, $setter:ident: $msb:expr, $lsb:expr, $count:expr; $($rest:tt)*) => {
       #[allow(unknown_lints)]
       #[allow(eq_op)]
       pub fn $setter(&mut self, index: usize, value: $t) {
           use $crate::BitRange;
           debug_assert!(index < $count);
           let width = $msb - $lsb + 1;
           let lsb = $lsb + index*width;
           let msb = lsb + width - 1;
           self.set_bit_range(msb, lsb, value);
       }
       simple_bitfield_fields!{$t, $($rest)*}
   };
   ($t:ty, $getter:ident, _: $msb:expr, $lsb:expr; $($rest:tt)*) => {
       pub fn $getter(&self) -> $t {
           use $crate::BitRange;
           self.bit_range($msb, $lsb)
       }
       simple_bitfield_fields!{$t, $($rest)*}
   };
   ($t:ty, $getter:ident, _: $msb:expr, $lsb:expr, $count:expr; $($rest:tt)*) => {
       #[allow(unknown_lints)]
       #[allow(eq_op)]
       pub fn $getter(&self, index: usize) -> $t {
           use $crate::BitRange;
           debug_assert!(index < $count);
           let width = $msb - $lsb + 1;
           let lsb = $lsb + index*width;
           let msb = lsb + width - 1;
           self.bit_range(msb, lsb)
       }
       simple_bitfield_fields!{$t, $($rest)*}
   };
   ($t:ty, $getter:ident, $setter:ident: $msb:expr, $lsb:expr; $($rest:tt)*) => {
       simple_bitfield_fields!{$t, $getter, _: $msb, $lsb; }
       simple_bitfield_fields!{$t, _, $setter: $msb, $lsb; }
       simple_bitfield_fields!{$t, $($rest)*}
   };
   ($t:ty, $getter:ident, $setter:ident: $msb:expr, $lsb:expr, $count:expr; $($rest:tt)*) => {
         simple_bitfield_fields!{$t, $getter, _: $msb, $lsb, $count; }
         simple_bitfield_fields!{$t, _, $setter: $msb, $lsb, $count; }
         simple_bitfield_fields!{$t, $($rest)*}
   };
}

#[macro_export]
macro_rules! simple_bitfield {
    ($name:ident, [$t:ty]; $($rest:tt)*) => {
        pub struct $name<T>(pub T);

        impl_bitrange_slice!($name, $t, u8);
        impl_bitrange_slice!($name, $t, u16);
        impl_bitrange_slice!($name, $t, u32);
        impl_bitrange_slice!($name, $t, u64);

        impl<T: AsMut<[$t]> + AsRef<[$t]>> $name<T> {
            simple_bitfield_fields!{u64, $($rest)*}
        }
    };
    ($name:ident, $t:ty; $($rest:tt)*) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(C)]
        pub struct $name(pub $t);

        impl $crate::BitRange<$t> for $name {
            fn bit_range(&self, msb: usize, lsb: usize) -> $t {
                self.0.bit_range(msb, lsb)
            }
            fn set_bit_range(&mut self, msb: usize, lsb: usize, value: $t) {
                self.0.set_bit_range(msb, lsb, value);
            }
        }

        impl $name {
            simple_bitfield_fields!{$t, $($rest)*}
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

macro_rules! impl_bitrange_for_u {
    ($t:ty) => {
        impl BitRange<$t> for $t {
            #[inline]
            fn bit_range(&self, msb: usize, lsb: usize) -> $t {
                let bit_len = size_of::<$t>()*8;
                (*self << (bit_len - msb - 1)) >> (bit_len - msb - 1 + lsb)
            }

            #[inline]
            fn set_bit_range(&mut self, msb: usize, lsb: usize, value: $t) {
                let bit_len = size_of::<$t>()*8;
                let mask: $t = !(0 as $t)
                    << (bit_len - msb - 1)
                    >> (bit_len - msb - 1 + lsb)
                        << (lsb);
                *self &= !mask;
                *self |= (value << lsb) & mask;
            }
        }
    }
}

impl_bitrange_for_u!{u8}
impl_bitrange_for_u!{u16}
impl_bitrange_for_u!{u32}
impl_bitrange_for_u!{u64}

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
