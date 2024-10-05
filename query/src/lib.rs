use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Attribute, Data, DeriveInput, Ident, ItemEnum, Lit, LitStr, Meta,
    MetaNameValue, Token,
};

/// Struct to represent the `path = "..."` inside the `#[get(...)]` attribute
struct GetAttributeArgs {
    path: LitStr,
}

impl Parse for GetAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse `path = "..."`
        let path_ident: Ident = input.parse()?;
        if path_ident != "path" {
            return Err(syn::Error::new_spanned(path_ident, "expected `path`"));
        }

        input.parse::<Token![=]>()?;
        let path_lit: LitStr = input.parse()?;

        Ok(GetAttributeArgs { path: path_lit })
    }
}

/// Helper function to extract the path from attributes
fn extract_path(attrs: &[Attribute]) -> Result<String> {
    for attr in attrs {
        // Check if the attribute is `#[get(...)]`
        if attr.path().is_ident("get") {
            // Parse the inside of `#[get(...)]` using our custom parser
            if let Ok(GetAttributeArgs { path }) = attr.parse_args() {
                return Ok(path.value());
            } else {
                return Err(syn::Error::new_spanned(
                    attr,
                    "Invalid syntax for `get` attribute",
                ));
            }
        }
    }
    Err(syn::Error::new_spanned(
        attrs.first().unwrap_or(&attrs[0]),
        "Attribute `#[get(path = \"...\")]` is required",
    ))
}

#[proc_macro_derive(QuerySchema, attributes(get))]
pub fn query_schema(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the enum
    let enum_name = &input.ident;
    let query_client_name = format_ident!("{}QueryClient", enum_name);

    // Ensure the input is an enum
    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        return TokenStream::from(quote! {
            compile_error!("QuerySchema can only be derived for enums.");
        });
    };

    // Collect match arms and generate structs and impls
    let mut match_arms = Vec::new();
    let mut structs_and_impls = Vec::new();
    let mut errors = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;

        // Create the name for the struct: VariantNameQuery
        let struct_name: Ident = Ident::new(&format!("{}Query", variant_name), variant_name.span());

        // Extract the path from attributes
        match extract_path(&variant.attrs) {
            Ok(path) => {
                // Generate a match arm for get_queryable
                match_arms.push(quote! {
                    Self::#variant_name => Box::new(#struct_name::new()),
                });

                // Generate the struct and impl for Queryable, including the new() method
                structs_and_impls.push(quote! {
                    pub struct #struct_name;

                    impl #struct_name {
                        pub fn new() -> Self {
                            #struct_name
                        }
                    }

                    impl Queryable for #struct_name {
                        fn query(&self) -> &str {
                            #path
                        }
                    }
                });
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    let errors = errors.into_iter().map(|error| error.to_compile_error());

    // Generate the final implementation
    let expanded = quote! {
        pub trait Queryable {
            fn query(&self) -> &str;
        }

        #(#errors)*

        impl #enum_name {
            pub fn get_queryable(&self) -> Box<dyn Queryable> {
                match self {
                    #(#match_arms)*
                }
            }
        }

        pub struct #query_client_name;

        impl #query_client_name {
            pub fn get_queryable(&self, variant: #enum_name) -> Box<dyn Queryable> {
                variant.get_queryable()
            }
        }

        #(#structs_and_impls)*
    };

    TokenStream::from(expanded)
}
