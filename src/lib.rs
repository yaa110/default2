use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Default2, attributes(default))]
pub fn default2(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct we're deriving `Default` for
    let struct_name = input.ident;

    // Get the fields of the struct
    let fields = match input.data {
        Data::Struct(struct_data) => struct_data.fields,
        _ => panic!("Default2 can only be derived for structs"),
    };

    // Generate code for setting default values for each field
    let field_defaults = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let attrs = &field.attrs;

        // Look for a `default` attribute on the field
        let default_value = attrs.iter().find_map(|attr| {
            if attr.path.is_ident("default") {
                // Parse the default value from the attribute
                let tokens = attr.tokens.clone();
                let result = syn::parse2::<syn::Expr>(tokens);
                match result {
                    Ok(lit) => Some(lit),
                    Err(e) => panic!(
                        "unable to parse default value for field {}: {}",
                        field_name, e
                    ),
                }
            } else {
                None
            }
        });

        // Generate code for setting the default value for the field
        match default_value {
            Some(lit) => {
                quote! {
                    #field_name: #lit
                }
            }
            None => {
                // If there's no `default` attribute on the field, use the `Default` trait
                quote! {
                    #field_name: Default::default()
                }
            }
        }
    });

    // Generate the final implementation of `Default` for the struct
    let output = quote! {
        impl Default for #struct_name {
            fn default() -> Self {
                Self {
                    #(#field_defaults),*
                }
            }
        }
    };

    // Return the generated code as a token stream
    output.into()
}
