use proc_macro::TokenStream;

use quote::quote;
use syn::{
    Ident, ItemStruct, LitStr, Path, Result, Token, Type,
    parse::{Parse, ParseStream},
};

use proc_macro2::Span;

struct Args {
    name: Option<LitStr>,
    backend: Path,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name = None;
        let mut backend = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if key == "name" {
                name = Some(input.parse()?);
            } else if key == "backend" {
                backend = Some(input.parse()?);
            } else {
                return Err(syn::Error::new(
                    key.span(),
                    "unknown argument, expected `name` or `backend`",
                ));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        let Some(backend) = backend else {
            return Err(syn::Error::new(
                Span::call_site(),
                "missing `backend`, write it like `#[table(backend = Pg)]`",
            ));
        };

        Ok(Self { name, backend })
    }
}

#[proc_macro_attribute]
pub fn table(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemStruct);
    let args = syn::parse_macro_input!(args as Args);

    let syn::Fields::Named(fields) = &input.fields else {
        return syn::Error::new_spanned(&input, "`#[table]` needs a struct with named fields")
            .to_compile_error()
            .into();
    };

    let ident = &input.ident;
    let backend = args.backend;
    let name = args
        .name
        .map_or_else(|| ident.to_string().to_lowercase(), |n| n.value());

    let field_idents: Vec<&Ident> = fields
        .named
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();
    let field_types: Vec<&Type> = fields.named.iter().map(|f| &f.ty).collect();
    let column_names: Vec<String> = field_idents.iter().map(ToString::to_string).collect();

    let mut column_defs = Vec::new();

    for (name, ty) in column_names.iter().zip(&field_types) {
        column_defs.push(quote! {
            ::sqlwright::ColumnDef {
                name: #name,
                sql_type: <#backend as ::sqlwright::HasSqlType<#ty>>::TYPE_NAME,
                nullable: <#backend as ::sqlwright::HasSqlType<#ty>>::NULLABLE,
            }
        });
    }

    quote! {
        #[derive(Clone, Copy, Debug)]
        pub struct #ident;

        impl ::sqlwright::Table for #ident {
            type Backend = #backend;
            const DEF: ::sqlwright::TableDef = ::sqlwright::TableDef {
                name: #name,
                columns: &[#(#column_defs),*],
            };
        }


        #[allow(non_upper_case_globals)]
        impl #ident {
            #(
                pub const #field_idents: ::sqlwright::Column<#ident, #field_types> =
                ::sqlwright::Column::new(#column_names);
            )*
        }

    }
    .into()
}
