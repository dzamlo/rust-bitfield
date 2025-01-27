use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{discouraged::Speculative, Parse, ParseStream},
    parse_macro_input, Attribute, Expr, Ident, Token, Type, Visibility,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(from);
    custom_keyword!(getter);
    custom_keyword!(into);
    custom_keyword!(mask);
    custom_keyword!(only);
    custom_keyword!(setter);
}

// We use Bitfield in the structs/enums names to avoid confusion with the types from syn/std

struct BitfieldMask {
    ident: Ident,
    ty: Type,
}

impl Parse for BitfieldMask {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let paren_content;
        parenthesized!(paren_content in input);
        let ty = paren_content.parse()?;
        Ok(BitfieldMask { ident, ty })
    }
}

enum BitfieldPosition {
    Bit(Expr),
    MsbLsb(Expr, Expr),
    MsbLsbCount(Expr, Expr, Expr),
}

impl Parse for BitfieldPosition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bit_position_1 = input.parse()?;
        if !input.peek(Token!(,)) {
            return Ok(BitfieldPosition::Bit(bit_position_1));
        }
        input.parse::<Token!(,)>()?;
        let bit_position_2 = input.parse()?;
        if !input.peek(Token!(,)) {
            return Ok(BitfieldPosition::MsbLsb(bit_position_1, bit_position_2));
        }
        input.parse::<Token!(,)>()?;
        let bit_position_3 = input.parse()?;

        Ok(BitfieldPosition::MsbLsbCount(
            bit_position_1,
            bit_position_2,
            bit_position_3,
        ))
    }
}

struct BitfieldField {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ty: Option<Type>,
    mask: Option<BitfieldMask>,
    from: bool,
    into: bool,
    from_into_ty: Option<Type>,
    getter: Ident,
    setter: Ident,
    bits_position: BitfieldPosition,
}

impl BitfieldField {
    fn ty_from(&self) -> Option<&Type> {
        if self.from {
            self.from_into_ty.as_ref()
        } else {
            self.ty.as_ref()
        }
    }

    fn ty_into(&self) -> Option<&Type> {
        if self.into {
            self.from_into_ty.as_ref()
        } else {
            self.ty.as_ref()
        }
    }
}

impl Parse for BitfieldField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let mut ty = None;
        let input_fork = input.fork();
        if let Ok(parsed_ty) = input_fork.parse() {
            if input_fork.peek(Token!(,)) && !input_fork.peek3(Token!(:)) {
                ty = Some(parsed_ty);
                input.advance_to(&input_fork);
                input.parse::<Token!(,)>()?;
            }
        };

        let mut mask = None;
        if input.peek(kw::mask) {
            input.parse::<kw::mask>()?;
            mask = Some(input.parse()?);
            input.parse::<Token!(,)>()?;
        }

        let from = input.peek(kw::from);
        if from {
            input.parse::<kw::from>()?;
        }

        let into = input.peek(kw::into);
        if into {
            input.parse::<kw::into>()?;
        }

        let mut from_into_ty = None;
        if from || into {
            from_into_ty = Some(input.parse()?);
            input.parse::<Token!(,)>()?;
        };

        let getter = input.call(Ident::parse_any)?;
        input.parse::<Token!(,)>()?;
        let setter = input.call(Ident::parse_any)?;
        input.parse::<Token!(:)>()?;
        let bits_position = input.parse()?;

        Ok(BitfieldField {
            attrs,
            vis,
            ty,
            mask,
            from,
            into,
            from_into_ty,
            getter,
            setter,
            bits_position,
        })
    }
}

enum BitfieldFieldLine {
    NewDefaultType(Type),
    Field(BitfieldField),
}

impl Parse for BitfieldFieldLine {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_fork = input.fork();

        Ok(match input_fork.parse() {
            // check if we have a single type statement either terminated by a semicolon or the
            // end of the stream, but do not consume the semicolon
            Ok(ty) if (input_fork.is_empty() || input_fork.peek(Token!(;))) => {
                input.advance_to(&input_fork);
                BitfieldFieldLine::NewDefaultType(ty)
            }
            _ => BitfieldFieldLine::Field(input.parse()?),
        })
    }
}

struct BitfieldFieldLines(Vec<BitfieldFieldLine>);

impl BitfieldFieldLines {
    fn into_fields(self) -> Vec<BitfieldField> {
        let mut result = vec![];
        let mut default_ty = None;
        for line in self.0 {
            match line {
                BitfieldFieldLine::NewDefaultType(ty) => default_ty = Some(ty),
                BitfieldFieldLine::Field(mut field) => {
                    if field.ty.is_none() {
                        field.ty = default_ty.clone();
                    }
                    result.push(field)
                }
            }
        }

        result
    }
}

impl Parse for BitfieldFieldLines {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let field_lines = input
            .parse_terminated(BitfieldFieldLine::parse, Token!(;))?
            .into_iter()
            .collect();

        Ok(BitfieldFieldLines(field_lines))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Only {
    Getter,
    Mask,
    Setter,
    None,
}

impl Only {
    fn getter_or_none(&self) -> bool {
        *self == Only::Getter || *self == Only::None
    }

    fn mask_or_none(&self) -> bool {
        *self == Only::Mask || *self == Only::None
    }

    fn setter_or_none(&self) -> bool {
        *self == Only::Setter || *self == Only::None
    }
}

struct BitfieldFieldsWithOnly {
    only: Only,
    fields: BitfieldFieldLines,
}

impl Parse for BitfieldFieldsWithOnly {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let only = if input.peek(kw::only) {
            input.parse::<kw::only>()?;
            let only = if input.peek(kw::getter) {
                input.parse::<kw::getter>()?;
                Only::Getter
            } else if input.peek(kw::setter) {
                input.parse::<kw::setter>()?;
                Only::Setter
            } else if input.peek(kw::mask) {
                input.parse::<kw::mask>()?;
                Only::Mask
            } else {
                return Err(input
                    .error("after the only keyword, either getter, mask or setter is expected"));
            };
            input.parse::<Token!(;)>()?;
            only
        } else {
            Only::None
        };
        let fields = input.parse()?;

        Ok(BitfieldFieldsWithOnly { only, fields })
    }
}

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
/// ```ignore
/// # use bitfield_macros::bitfield_fields;
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
pub fn bitfield_fields(input: TokenStream) -> TokenStream {
    let fields_with_only = parse_macro_input!(input as BitfieldFieldsWithOnly);

    let only = fields_with_only.only;
    let fields = fields_with_only.fields.into_fields();

    let getters = if only.getter_or_none() {
        generate_getters(&fields)
    } else {
        quote! {}
    };

    let setters = if only.setter_or_none() {
        generate_setters(&fields)
    } else {
        quote! {}
    };

    let masks = if only.mask_or_none() {
        generate_masks(&fields)
    } else {
        quote! {}
    };

    let expanded = quote! {
        #getters
        #setters
        #masks
    };
    TokenStream::from(expanded)
}
static MISSING_TYPE_ERROR_MESSAGE: &str =
    "For non single bit field, you need to either specify a default type or a type for each field";

fn generate_getters(fields: &[BitfieldField]) -> proc_macro2::TokenStream {
    let getters = fields.iter().flat_map(|field| {
        if field.getter != "_" {
            let attrs = &field.attrs;
            let vis = &field.vis;
            let getter = &field.getter;
            Some(match &field.bits_position {
                BitfieldPosition::Bit(bit) => {
                    quote! {
                        #(#attrs)*
                        #vis fn #getter(&self) -> bool {
                            use ::bitfield::Bit;
                            self.bit(#bit)
                         }
                    }
                }
                BitfieldPosition::MsbLsb(msb, lsb) => {
                    let ty = field.ty.as_ref().expect(MISSING_TYPE_ERROR_MESSAGE);
                    let ty_into = field.ty_into().unwrap();
                    quote! {
                        #(#attrs)*
                        #vis fn #getter(&self) -> #ty_into {
                            use ::bitfield::BitRange;
                            let raw_value: #ty = self.bit_range(#msb, #lsb);
                            ::bitfield::Into::into(raw_value)
                        }
                    }
                }
                BitfieldPosition::MsbLsbCount(msb, lsb, count) => {
                    let ty = field.ty.as_ref().expect(MISSING_TYPE_ERROR_MESSAGE);
                    let ty_into = field.ty_into().unwrap();
                    quote! {
                        #(#attrs)*
                        #vis fn #getter(&self, index: usize) -> #ty_into {
                            use ::bitfield::BitRange;
                            __bitfield_debug_assert!(index < #count);
                            #[allow(clippy::eq_op)]
                            #[allow(clippy::identity_op)]
                            let width = #msb - #lsb + 1;
                            let lsb = #lsb + index*width;
                            let msb = lsb + width - 1;
                            let raw_value: #ty = self.bit_range(msb, lsb);
                            ::bitfield::Into::into(raw_value)
                        }
                    }
                }
            })
        } else {
            None
        }
    });
    quote! { #(#getters)* }
}

fn generate_setters(fields: &[BitfieldField]) -> proc_macro2::TokenStream {
    let setters = fields.iter().flat_map(|field| {
        if field.setter != "_" {
            let attrs = &field.attrs;
            let vis = &field.vis;
            let setter = &field.setter;
            Some(match &field.bits_position {
                BitfieldPosition::Bit(bit) => {
                    quote! {
                        #(#attrs)*
                        #vis fn #setter(&mut self, value: bool) {
                            use ::bitfield::BitMut;
                            self.set_bit(#bit, value);
                         }
                    }
                }
                BitfieldPosition::MsbLsb(msb, lsb) => {
                    let ty = field.ty.as_ref().expect(MISSING_TYPE_ERROR_MESSAGE);
                    let ty_from = field.ty_from().unwrap();
                    quote! {
                        #(#attrs)*
                        #vis fn #setter(&mut self, value: #ty_from) {
                            use ::bitfield::BitRangeMut;
                            self.set_bit_range(#msb, #lsb, ::bitfield::Into::<#ty>::into(value));
                         }
                    }
                }
                BitfieldPosition::MsbLsbCount(msb, lsb, count) => {
                    let ty = field.ty.as_ref().expect(MISSING_TYPE_ERROR_MESSAGE);
                    let ty_from = field.ty_from().unwrap();
                    quote! {
                        #(#attrs)*
                        #vis fn #setter(&mut self, index: usize, value: #ty_from) {
                            use ::bitfield::BitRangeMut;
                            __bitfield_debug_assert!(index < #count);
                            #[allow(clippy::eq_op)]
                            #[allow(clippy::identity_op)]
                            let width = #msb - #lsb + 1;
                            let lsb = #lsb + index*width;
                            let msb = lsb + width - 1;
                            self.set_bit_range(msb, lsb, ::bitfield::Into::<#ty>::into(value));
                         }
                    }
                }
            })
        } else {
            None
        }
    });
    quote! { #(#setters)* }
}

fn generate_masks(fields: &[BitfieldField]) -> proc_macro2::TokenStream {
    let masks = fields.iter().flat_map(|field| {
        if let Some(mask) = &field.mask {
            let vis = &field.vis;
            let mask_ident = &mask.ident;
            let mask_ty = &mask.ty;
            Some(match &field.bits_position {
                BitfieldPosition::Bit(bit) => {
                    quote! {#vis const #mask_ident: #mask_ty = 1 << (#bit);}
                }
                BitfieldPosition::MsbLsb(msb, lsb) => {
                    quote! {
                        #vis const #mask_ident: #mask_ty = {
                            let msb = #msb;
                            let lsb = #lsb;
                            let mut i = lsb;
                            let mut acc = 0;
                            while i <= msb {
                                 acc |= 1<<i;
                                 i += 1;
                            }
                            acc
                        };
                    }
                }
                BitfieldPosition::MsbLsbCount(msb, lsb, count) => {
                    quote! {
                        #vis const #mask_ident: #mask_ty = {
                            let msb = #msb;
                            let lsb = #lsb;
                            let count = #count;
                            let width = msb - lsb;
                            let full_msb = msb + width * count;
                            let mut i = lsb;
                            let mut acc = 0;
                            while i <= full_msb {
                                acc |= 1<<i;
                                i += 1;
                            }
                            acc
                        };
                    }
                }
            })
        } else {
            None
        }
    });
    quote! { #(#masks)* }
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
