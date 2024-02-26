use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

// Based on https://www.shuttle.rs/blog/2022/12/23/procedural-macros
// https://www.priver.dev/blog/rust/procedural-macros

#[proc_macro_derive(PostgresType)]
pub fn deserialize_postgres_type(input: TokenStream) -> TokenStream {
    // parse the token into a tree
    let input: DeriveInput = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let Data::Struct(struct_data) = input.data else {
        unimplemented!("Only structs are supported");
    };

    let de_fields = struct_data.fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();

        quote! { #ident: value.try_get(stringify!(#ident))?, }
    });

    let expanded = quote! {
        #[automatically_derived]
        impl TryFrom<Row> for #name {
            type Error = anyhow::Error;

            fn try_from(value: Row) -> Result<Self> {
                Ok(Self {
                    #( #de_fields )*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
