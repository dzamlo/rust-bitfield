use proc_macro::TokenStream;
use quote::quote;

/// Declares the fields of struct.
///
/// This macro will generate the methods to access the fields of a bitfield. It must be called
/// from an `impl` block for a type that implements the `BitRange` and/or the `Bit` traits
/// (which traits are required depending on what type of fields are used).
///
/// The syntax of this macro is composed of declarations ended by semicolons. There are two types
/// of declarations: default type, and fields.
///
/// A default type is just a type followed by a semicolon. This will affect all the following field
/// declarations.
///
/// A field declaration is composed of the following:
///
/// * Optional attributes (`#[...]`), documentation comments (`///`) are attributes;
/// * An optional pub keyword to make the methods public
/// * An optional type followed by a comma
/// * Optionally, the word `into` followed by a type, followed by a comma
/// * The getter and setter idents, separated by a comma
/// * A colon
/// * One to three expressions of type `usize`
///
/// The attributes and pub will be applied to the two methods generated.
///
/// If the `into` part is used, the getter will convert the field after reading it.
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
/// # use bitfield_macros::{bitfield_bitrange, bitfield_fields};
/// # fn main() {}
/// # struct FooBar(u64);
/// # bitfield_bitrange!{struct FooBar(u64)}
/// # impl From<u32> for FooBar{ fn from(_: u32) -> FooBar {unimplemented!()}}
/// # impl From<FooBar> for u32{ fn from(_: FooBar) -> u32 {unimplemented!()}}
/// # impl FooBar {
/// bitfield_fields!{
///     // The default type will be `u64
///     u64;
///     // filed1 is read-write, public, the methods are inline
///     #[inline]
///     pub field1, set_field1: 10, 0;
///     // `field2` is  read-only, private, and of type bool.
///     field2, _ : 0;
///     // `field3` will be read as an `u32` and then converted to `FooBar`.
///     // The setter is not affected, it still need an `u32` value.
///     u32, into FooBar, field3, set_field3: 10, 0;
///     // `field4` will be read as an `u32` and then converted to `FooBar`.
///     // The setter will take a `FooBar`, and converted back to an `u32`.
///     u32, from into FooBar, field4, set_field4: 10, 0;
///     // `field5` will be read as an `u32` and then converted to `FooBar`.
///     // The setter will take a `FooBar`, and converted back to an `u32`.
///     // The struct will have an associated constant `FIELD5_MASK` of type u64
///     //with the bits of field5 set
///     u32, mask FIELD5_MASK(u64), from into FooBar, field5, set_field5: 10, 0;
/// }
/// # }
/// ```
#[proc_macro]
pub fn bitfield_fields(_input: TokenStream) -> TokenStream {
    let expanded = quote! {};
    TokenStream::from(expanded)
}

/// Combines `bitfield_bitrange` and `bitfield_fields`.
///
/// The syntax of this macro is the syntax of a tuple struct, including attributes and
/// documentation comments, followed by a semicolon, some optional elements, and finally the fields
/// as described in the `bitfield_fields` documentation.
///
/// The first optional element is `no default BitRange;`. With that, no implementation of
/// `BitRange` will be generated.
///
/// The second optional element is a set of lines of the form `impl <Trait>;`. The following traits are supported:
/// * `Debug`; This will generate an implementation of `fmt::Debug` with the `bitfield_debug` macro.
/// * `BitAnd`, `BitOr`, `BitXor`; These will generate implementations of the relevant `ops::Bit___` and `ops::Bit___Assign` traits.
/// * `new`; This will generate a constructor that calls all of the bitfield's setter methods with an argument of the appropriate type
/// * `new{constructor_name(setter_name: setter_type, ...)}`; This will generate a constructor that calls a given subset of the bitfield's setter methods
///
/// The difference with calling those macros separately is that `bitfield_fields` is called
/// from an appropriate `impl` block. If you use the non-slice form of `bitfield_bitrange`, the
/// default type for `bitfield_fields` will be set to the wrapped fields.
///
/// See the documentation of these macros for more information on their respective syntax.
///
/// # Example
///
/// ```rust
/// # use bitfield_macros::bitfield;
/// # fn main() {}
/// bitfield!{
///   pub struct BitField1(u16);
///   impl Debug;
///   // The fields default to u16
///   field1, set_field1: 10, 0;
///   pub field2, _ : 12, 3;
/// }
/// ```
///
/// or with a custom `BitRange` and `BitRangeMut` implementation :
/// ```ignore
/// bitfield!{
///   pub struct BitField1(u16);
///   no default BitRange;
///   impl Debug;
///   impl BitAnd;
///   u8;
///   field1, set_field1: 10, 0;
///   pub field2, _ : 12, 3;
/// }
/// impl BitRange<u8> for BitField1 {
///     fn bit_range(&self, msb: usize, lsb: usize) -> u8 {
///         let width = msb - lsb + 1;
///         let mask = (1 << width) - 1;
///         ((self.0 >> lsb) & mask) as u8
///     }
/// }
/// impl BitRangeMut<u8> for BitField1 {
///     fn set_bit_range(&mut self, msb: usize, lsb: usize, value: u8) {
///         self.0 = (value as u16) << lsb;
///     }
/// }
/// ```
#[proc_macro]
pub fn bitfield(_input: TokenStream) -> TokenStream {
    let expanded = quote! {};
    TokenStream::from(expanded)
}
