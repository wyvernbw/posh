use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Result};

use crate::utils::{get_domain_param, SpecializedTypeGenerics, StructFields};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;

    let generics_view_type = get_domain_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let ty_generics_sl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Sl), ident, &input.generics)?;
    let ty_generics_gl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Gl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();

    Ok(quote! {
        // Implement `Fragment<D>` for the struct.
        unsafe impl #impl_generics ::posh::Fragment<#generics_view_type>
        for #ident #ty_generics
        #where_clause
        {
            type Sl = #ident #ty_generics_sl;
            type Gl = #ident #ty_generics_gl;

            fn visit<'a>(
                &'a self,
                path: &str,
                visitor: &mut impl ::posh::internal::FragmentVisitor<'a, #generics_view_type>,
            ) {
                #(
                    visitor.accept(
                        &::posh::internal::join_ident_path(path, #field_strings),
                        &self.#field_idents,
                    );
                )*
            }
        }

        // Check that all field types implement `Fragment<D>`.
        const _: fn() = || {
            fn check_field<D, T>()
            where
                D: ::posh::FragmentDom,
                T: ::posh::Fragment<D>,
            {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#generics_view_type, #field_types>();
                )*
            }
        };
    })
}