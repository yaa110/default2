//! A convenient macro to implement `Default` for structs using field initializers.
//!
//! See the [`default!`] macro for usage details.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Expr, Generics, Ident, Result, Token, Type, Visibility, WhereClause,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// A field in the struct, with an optional default value. Used for parsing.
struct FieldWithValue {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    ty: Type,
    default: Option<Expr>,
}

impl Parse for FieldWithValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse::<Type>()?;
        let default = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse::<Expr>()?)
        } else {
            None
        };
        Ok(FieldWithValue {
            attrs,
            vis,
            ident,
            ty,
            default,
        })
    }
}

/// A struct definition with fields that can have default values. Used for parsing.
struct StructDef {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    generics: Generics,
    fields: Punctuated<FieldWithValue, Token![,]>,
}

impl Parse for StructDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;
        input.parse::<Token![struct]>()?;
        let ident = input.parse::<Ident>()?;
        let mut generics = input.parse::<Generics>()?;
        if input.peek(Token![where]) {
            generics.where_clause = Some(input.parse::<WhereClause>()?);
        }

        let content;
        syn::braced!(content in input);
        let fields = content.parse_terminated(FieldWithValue::parse, Token![,])?;

        Ok(StructDef {
            attrs,
            vis,
            ident,
            generics,
            fields,
        })
    }
}

/// A function-like procedural macro to implement `Default` for a struct.
///
/// This macro allows you to define a struct and provide default values
/// for its fields using a convenient `=` syntax. Any attributes placed on the
/// struct, such as `#[derive(Debug)]`, will be passed through.
///
/// Fields without an initializer will receive their default value from
/// `std::default::Default::default()`.
///
/// # Example
///
/// ```rust,ignore
/// use default2::default;
///
/// default! {
///     #[derive(Debug, PartialEq)]
///     struct MyStruct {
///         id: i32 = 42,
///         name: String,
///     }
/// }
///
/// let s = MyStruct::default();
/// assert_eq!(s.id, 42);
/// assert_eq!(s.name, String::new());
/// ```
#[proc_macro]
pub fn default(item: TokenStream) -> TokenStream {
    let input = match syn::parse::<StructDef>(item) {
        Ok(input) => input,
        Err(e) => return e.to_compile_error().into(),
    };

    let struct_name = &input.ident;
    let vis = &input.vis;

    let mut use_const_default = false;
    let mut attrs = input.attrs.clone();
    attrs.retain(|attr| {
        if attr.path().is_ident("const_default") {
            use_const_default = true;
            false // remove it
        } else {
            true // keep it
        }
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields_no_defaults = input.fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_ty = &f.ty;
        let field_vis = &f.vis;
        let field_attrs = &f.attrs;
        quote! { #(#field_attrs)* #field_vis #field_name: #field_ty }
    });

    let field_defaults = input.fields.iter().map(|f| {
        let field_name = &f.ident;
        if let Some(default_expr) = &f.default {
            quote! { #field_name: #default_expr }
        } else {
            quote! { #field_name: std::default::Default::default() }
        }
    });

    let const_default_impl = if use_const_default {
        let const_field_defaults = field_defaults.clone();
        quote! {
            impl #impl_generics #struct_name #ty_generics #where_clause {
                /// Creates a new instance of the struct with constant default values.
                ///
                /// This function is only available when the `#[const_default]` attribute is used.
                /// The caller is responsible for ensuring that all default value expressions
                /// are valid in a `const` context.
                pub const fn const_default() -> Self {
                    Self {
                        #(#const_field_defaults,)*
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #(#attrs)*
        #vis struct #struct_name #ty_generics #where_clause {
            #(#fields_no_defaults,)*
        }

        impl #impl_generics std::default::Default for #struct_name #ty_generics #where_clause {
            fn default() -> Self {
                Self {
                    #(#field_defaults,)*
                }
            }
        }

        #const_default_impl
    };

    TokenStream::from(expanded)
}
