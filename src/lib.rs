use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{
    parse,
    token::{AndEq, CaretEq, MinusEq, OrEq, PercentEq, PlusEq, ShlEq, ShrEq, SlashEq, StarEq},
    BinOp, DeriveInput, Ident,
};

/*
 * This macro generates a trait that implements an operator from an operator assignment
 * and the "Clone" trait.
 */

fn op_from_op_assign(input: TokenStream, op_name: &str, operator: BinOp) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let trait_name = Ident::new(&format!("{}", op_name), Span::call_site());
    let function_name = Ident::new(&format!("{}", op_name.to_lowercase()), Span::call_site());
    let gen = quote! {
        impl #impl_generics #trait_name for #name #ty_generics #where_clause {
            type Output = Self;
            fn #function_name (self, rhs: Self) -> Self {
                let mut ret = self.clone();
                ret #operator rhs;
                return ret;
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Add)]
pub fn add_from_add_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(input, "Add", syn::BinOp::AddAssign(PlusEq::default()))
}

#[proc_macro_derive(Sub)]
pub fn sub_from_sub_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(input, "Sub", syn::BinOp::SubAssign(MinusEq::default()))
}

#[proc_macro_derive(Mul)]
pub fn mul_from_mul_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(input, "Mul", syn::BinOp::MulAssign(StarEq::default()))
}

#[proc_macro_derive(Div)]
pub fn div_from_div_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(input, "Div", syn::BinOp::DivAssign(SlashEq::default()))
}

#[proc_macro_derive(Rem)]
pub fn rem_from_rem_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(input, "Rem", syn::BinOp::RemAssign(PercentEq::default()))
}

#[proc_macro_derive(BitXor)]
pub fn bitxor_from_bitxor_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(
        input,
        "BitXor",
        syn::BinOp::BitXorAssign(CaretEq::default()),
    )
}

#[proc_macro_derive(BitAnd)]
pub fn bitand_from_bitand_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(input, "BitAnd", syn::BinOp::BitAndAssign(AndEq::default()))
}

#[proc_macro_derive(BitOr)]
pub fn bitor_from_bitor_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(input, "BitOr", syn::BinOp::BitOrAssign(OrEq::default()))
}

#[proc_macro_derive(Shl)]
pub fn shl_from_shl_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(input, "Shl", syn::BinOp::ShlAssign(ShlEq::default()))
}

#[proc_macro_derive(Shr)]
pub fn shr_from_shr_assign(input: TokenStream) -> TokenStream {
    op_from_op_assign(input, "Shr", syn::BinOp::ShrAssign(ShrEq::default()))
}

/*
 * These macros generate a "From" trait for integral types, assuming the type has a "From" trait
 * defined for a 64-bit integral type.
 */

#[proc_macro_derive(FromUnsigned)]
pub fn from_unsigned(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = quote! {

        impl #impl_generics From<u8> for #name #ty_generics #where_clause {
            type Output = Self;
            fn from(input: u8) -> Self {
                return Self::from(input as u64);
            }
        }

        impl #impl_generics From<u16> for #name #ty_generics #where_clause {
            type Output = Self;
            fn from(input: u16) -> Self {
                return Self::from(input as u64);
            }
        }

        impl #impl_generics From<u32> for #name #ty_generics #where_clause {
            type Output = Self;
            fn from(input: u32) -> Self {
                return Self::from(input as u64);
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(FromSigned)]
pub fn from_signed(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = quote! {

        impl #impl_generics From<i8> for #name #ty_generics #where_clause {
            type Output = Self;
            fn from(input: i8) -> Self {
                return Self::from(input as i64);
            }
        }

        impl #impl_generics From<i16> for #name #ty_generics #where_clause {
            type Output = Self;
            fn from(input: i16) -> Self {
                return Self::from(input as i64);
            }
        }

        impl #impl_generics From<i32> for #name #ty_generics #where_clause {
            type Output = Self;
            fn from(input: i32) -> Self {
                return Self::from(input as i64);
            }
        }
    };
    gen.into()
}
