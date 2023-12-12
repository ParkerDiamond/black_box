use proc_macro::TokenStream as CompilerTokenStream;
use proc_macro2::{Span, TokenStream};

use quote::quote;
use syn::{parse, BinOp, DeriveInput, Ident, Token};

//
// This macro generates a trait that implements an operator from an operator assignment
// and the "Clone" trait.
//

fn op_from_op_assign(input: CompilerTokenStream, op_name: &str, operator: BinOp) -> TokenStream {
    let DeriveInput {
        ident: name,
        generics,
        ..
    } = parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let trait_name = Ident::new(op_name, Span::call_site());
    let function_name = Ident::new(&op_name.to_lowercase(), Span::call_site());

    quote! {
        impl #impl_generics #trait_name for #name #ty_generics #where_clause {
            type Output = Self;

            fn #function_name (self, rhs: Self) -> Self {
                let mut ret = self.clone();

                ret #operator rhs;

                ret
            }
        }
    }
}

macro_rules! derive_for_assign_op {
    ($name:ident for $t:tt) => {
        paste::paste! {
            #[proc_macro_derive($name)]
            pub fn [<$name:lower _for_ $name:lower _assign>](input: CompilerTokenStream) -> CompilerTokenStream {
                op_from_op_assign(input, stringify!($name), syn::BinOp::[<$name Assign>](<Token![$t]>::default())).into()
            }
        }
    };
}

derive_for_assign_op!(Add for +=);
derive_for_assign_op!(Sub for -=);
derive_for_assign_op!(Mul for *=);
derive_for_assign_op!(Rem for %=);
derive_for_assign_op!(BitXor for ^=);
derive_for_assign_op!(BitAnd for &=);
derive_for_assign_op!(BitOr for |=);
derive_for_assign_op!(Shl for <<=);
derive_for_assign_op!(Shr for >>=);

//
// These macros generate a "From" trait for integral types, assuming the type has a "From" trait
// defined for a 64-bit integral type.
//

#[proc_macro_derive(FromUnsigned)]
pub fn from_unsigned(input: CompilerTokenStream) -> CompilerTokenStream {
    let DeriveInput {
        ident: name,
        generics,
        ..
    } = parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics From<u8> for #name #ty_generics #where_clause {
            fn from(input: u8) -> Self {
                Self::from(input as u64)
            }
        }

        impl #impl_generics From<u16> for #name #ty_generics #where_clause {
            fn from(input: u16) -> Self {
                Self::from(input as u64)
            }
        }

        impl #impl_generics From<u32> for #name #ty_generics #where_clause {
            fn from(input: u32) -> Self {
                Self::from(input as u64)
            }
        }
    }
    .into()
}

#[proc_macro_derive(FromSigned)]
pub fn from_signed(input: CompilerTokenStream) -> CompilerTokenStream {
    let DeriveInput {
        ident: name,
        generics,
        ..
    } = parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics From<i8> for #name #ty_generics #where_clause {
            fn from(input: i8) -> Self {
                Self::from(input as i64)
            }
        }

        impl #impl_generics From<i16> for #name #ty_generics #where_clause {
            fn from(input: i16) -> Self {
                Self::from(input as i64)
            }
        }

        impl #impl_generics From<i32> for #name #ty_generics #where_clause {
            fn from(input: i32) -> Self {
                Self::from(input as i64)
            }
        }
    }
    .into()
}
