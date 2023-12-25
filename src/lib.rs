use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::{parse, BinOp, DeriveInput, Ident, Token};

macro_rules! bind_parse_for_impl {
    ({ ident: $name:ident, generics: ($impl_generics:ident, $ty_generics:ident, $where_clause:ident) } in $input:ident) => {
        let DeriveInput {
            ident: $name,
            generics,
            ..
        } = parse($input).unwrap();

        let ($impl_generics, $ty_generics, $where_clause) = generics.split_for_impl();
    };
}

// This macro generates a trait that implements an operator from an operator assignment
// and the "Clone" trait.
macro_rules! derive_for_assign_op {
    ($name:ident for $t:tt) => {
        paste::paste! {
            #[proc_macro_derive($name)]
            pub fn [<$name:lower _for_ $name:lower _assign>](input: TokenStream) -> TokenStream {
                bind_parse_for_impl!({ ident: name, generics: (impl_generics, ty_generics, where_clause) } in input);

                let trait_name = Ident::new(stringify!($name), Span::call_site());
                let function_name = Ident::new(stringify!([<$name:lower>]), Span::call_site());
                let operator = BinOp::[<$name Assign>](<Token![$t]>::default());

                quote::quote! {
                    impl #impl_generics #trait_name for #name #ty_generics #where_clause {
                        type Output = Self;

                        fn #function_name (self, rhs: Self) -> Self {
                            let mut value = self.clone();
                            value #operator rhs;
                            value
                        }
                    }
                }.into()
            }
        }
    };
}

derive_for_assign_op!(Add for +=);
derive_for_assign_op!(Sub for -=);
derive_for_assign_op!(Mul for *=);
derive_for_assign_op!(Div for /=);
derive_for_assign_op!(Rem for %=);
derive_for_assign_op!(BitXor for ^=);
derive_for_assign_op!(BitAnd for &=);
derive_for_assign_op!(BitOr for |=);
derive_for_assign_op!(Shl for <<=);
derive_for_assign_op!(Shr for >>=);

// These macros generate a "From" trait for integral types, assuming the type has a "From" trait
// defined for a 64-bit integral type.
macro_rules! derive_from_impls {
    ($name:ident for ($($t:ty),+) as $as_ty:ty) => {
        paste::paste! {
            #[proc_macro_derive([<From $name>])]
            pub fn [<from_ $name:lower>](input: TokenStream) -> TokenStream {
                bind_parse_for_impl!({ ident: name, generics: (impl_generics, ty_generics, where_clause) } in input);

                quote::quote! {$(
                    impl #impl_generics From<$t> for #name #ty_generics #where_clause {
                        fn from(input: $t) -> Self { Self::from(input as $as_ty) }
                    }
                )*}.into()
            }
        }
    }
}

derive_from_impls!(Unsigned for (u8, u16, u32) as u64);
derive_from_impls!(Signed for (i8, i16, i32) as i64);

// These macros generate a "PartialEq" trait for integral types, assuming the type has a "PartialEq" trait
// defined for a 64-bit integral type.
macro_rules! derive_partial_eq_impls {
    ($name:ident for ($($t:ty),+) as $as_ty:ty) => {
        paste::paste! {
            #[proc_macro_derive([<PartialEq $name>])]
            pub fn [<eq $name:lower>](input: TokenStream) -> TokenStream {
                bind_parse_for_impl!({ ident: name, generics: (impl_generics, ty_generics, where_clause) } in input);

                quote::quote! {$(
                    impl #impl_generics PartialEq<$t> for #name #ty_generics #where_clause {
                        fn eq(&self, other: &$t) -> bool { self == &(*other as $as_ty) }
                    }
                )*}.into()
            }
        }
    }
}

derive_partial_eq_impls!(Unsigned for (u8, u16, u32) as u64);
derive_partial_eq_impls!(Signed for (i8, i16, i32) as i64);
