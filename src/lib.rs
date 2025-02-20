use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, parse_macro_input};

#[proc_macro_derive(Default, attributes(default))]
pub fn default2(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct.
    let name = &input.ident;

    // Extract the fields of the struct.
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("Default can only be used with structs"),
    };

    // Generate the implementation of the Default trait.
    let field_defaults = generate_field_defaults(fields);

    let expanded = quote! {
        impl Default for #name {
            fn default() -> Self {
                #name {
                    #(#field_defaults,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn generate_field_defaults(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let default_attr = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("default"));

            match default_attr {
                Some(attr) => {
                    let default_value: Expr = attr.parse_args().expect("Invalid default value");
                    quote! { #field_name: #default_value }
                }
                None => quote! { #field_name: Default::default() },
            }
        })
        .collect()
}
