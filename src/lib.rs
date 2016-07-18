#[macro_export]
macro_rules! simple_bitfield_field {
   ($name:ident, $t:ty,) => {};
   ($name:ident, $t:ty, _, $setter:ident: $msb:expr, $lsb:expr, $($rest:tt)*) => {
       impl $name {
           pub fn $setter(&mut self, value: $t) {
               self.set_range_($msb, $lsb, value);
           }
       }
       simple_bitfield_field!{$name, $t, $($rest)*}
   };
   ($name:ident, $t:ty, _, $setter:ident: $msb:expr, $lsb:expr; $count:expr, $($rest:tt)*) => {
       impl $name {
           pub fn $setter(&mut self, index: usize, value: $t) {
               debug_assert!(index < $count);
               let width = $msb - $lsb + 1;
               let lsb = $lsb + index*width;
               let msb = lsb + width - 1;
               self.set_range_(msb, lsb, value);
           }
       }
       simple_bitfield_field!{$name, $t, $($rest)*}
   };
   ($name:ident, $t:ty, $getter:ident, _: $msb:expr, $lsb:expr, $($rest:tt)*) => {
       impl $name {
           pub fn $getter(&self) -> $t {
               self.get_range_($msb, $lsb)
           }
       }
       simple_bitfield_field!{$name, $t, $($rest)*}
   };
   ($name:ident, $t:ty, $getter:ident, _: $msb:expr, $lsb:expr; $count:expr, $($rest:tt)*) => {
       impl $name {
           pub fn $getter(&self, index: usize) -> $t {
               debug_assert!(index < $count);
               let width = $msb - $lsb + 1;
               let lsb = $lsb + index*width;
               let msb = lsb + width - 1;
               self.get_range_(msb, lsb)
           }
       }
       simple_bitfield_field!{$name, $t, $($rest)*}
   };
   ($name:ident, $t:ty, $getter:ident, $setter:ident: $msb:expr, $lsb:expr, $($rest:tt)*) => {
       simple_bitfield_field!{$name, $t, $getter, _: $msb, $lsb, }
       simple_bitfield_field!{$name, $t, _, $setter: $msb, $lsb, }
       simple_bitfield_field!{$name, $t, $($rest)*}
   };
   ($name:ident, $t:ty, $getter:ident, $setter:ident: $msb:expr, $lsb:expr; $count:expr, $($rest:tt)*) => {
         simple_bitfield_field!{$name, $t, $getter, _: $msb, $lsb; $count, }
         simple_bitfield_field!{$name, $t, _, $setter: $msb, $lsb; $count, }
         simple_bitfield_field!{$name, $t, $($rest)*}
   };
}


#[macro_export]
macro_rules! simple_bitfield {
    ($name:ident, $t:ty, $($rest:tt)*) => {
         #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
         #[repr(C)]
         pub struct $name(pub $t);
         impl $name {
             fn get_range_(&self, msb: usize, lsb: usize) -> $t {
                 let bit_len = mem::size_of::<$t>()*8;
                 (self.0 << (bit_len - msb - 1)) >> (bit_len - msb - 1 + lsb)
             }

             fn set_range_(&mut self, msb: usize, lsb: usize, value: $t) {
                 let bit_len = mem::size_of::<$t>()*8;
                 let mask: $t = !(0 as $t) << (bit_len - msb - 1) >> (bit_len - msb - 1 + lsb) << (lsb);
                 self.0 &= !mask;
                 self.0 |= (value << lsb) & mask;
             }
         }
         simple_bitfield_field!{$name, $t, $($rest)*}
    }
}
