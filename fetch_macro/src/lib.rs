use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Attribute, Data, DeriveInput, Ident, LitStr, Token, Type,
};

struct FetchAttributeArgs {
    path: LitStr,
    query: Option<Type>,
    body: Option<Type>,
    res: Option<Type>,
}

impl Parse for FetchAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let path: LitStr = input.parse()?;
        let mut query = None;
        let mut body = None;
        let mut res = None;

        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match ident.to_string().as_str() {
                "query" => {
                    let ty: Type = input.parse()?;
                    query = Some(ty);
                }
                "body" => {
                    let ty: Type = input.parse()?;
                    body = Some(ty);
                }
                "res" => {
                    let ty: Type = input.parse()?;
                    res = Some(ty);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident.clone(),
                        format!("Unknown argument `{}`", ident),
                    ));
                }
            }
        }

        Ok(FetchAttributeArgs {
            path,
            query,
            body,
            res,
        })
    }
}

static HTTP_VERBS: [&str; 5] = ["GET", "POST", "PUT", "PATCH", "DELETE"];

fn extract_attrs(attrs: &[Attribute]) -> Result<(String, Type, Type, Type)> {
    for attr in attrs {
        if let Some(ident) = attr.path().get_ident() {
            if HTTP_VERBS.contains(&ident.to_string().to_uppercase().as_str()) {
                let FetchAttributeArgs {
                    path,
                    query,
                    body,
                    res,
                } = attr.parse_args()?;

                let path = path.value();
                let query = query.unwrap_or_else(|| syn::parse_quote! { () });
                let body = body.unwrap_or_else(|| syn::parse_quote! { () });
                let res = res.unwrap_or_else(|| syn::parse_quote! { () });

                return Ok((path, query, body, res));
            }
        }
    }

    Err(syn::Error::new_spanned(
        attrs.first().unwrap_or(&attrs[0]),
        "Attribute `#[verb(\"...\")]` is required",
    ))
}

#[proc_macro_derive(FetchSchema, attributes(get, post, put, patch, delete))]
pub fn fetch_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let fetch_client_name = format_ident!("{}FetchClient", enum_name);

    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        return TokenStream::from(quote! {
            compile_error!("FetchSchema can only be derived for enums.");
        });
    };

    let mut methods = Vec::new();
    let mut structs = Vec::new();
    let mut hooks = Vec::new();
    let mut errors = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;
        let method_params_struct_name = format_ident!("{}Params", variant_name);
        let method_name = format_ident!("{}", variant_name.to_string().to_snake_case());
        let hook_name = format_ident!("use_{}", method_name);

        match extract_attrs(&variant.attrs) {
            Ok((path, query, body, res)) => {
                structs.push(quote! {
                    #[derive(Default)]
                    pub struct #method_params_struct_name {
                        query: #query,
                        body: #body,
                    }
                });

                methods.push(quote! {
                    pub fn #method_name(&self, params: #method_params_struct_name) -> #res {
                        #path
                    }
                });

                hooks.push(quote! {
                    #[hook]
                    pub fn #hook_name(params: #method_params_struct_name) -> #res {
                        #path
                    }
                });
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    let errors = errors.into_iter().map(|error| error.to_compile_error());

    let expanded = quote! {
        use yew::hook;

        #(#structs)*
        #(#errors)*

        pub struct #fetch_client_name {
            base_url: String,
        }

        impl #fetch_client_name {
            pub fn new(base_url: String) -> Self {
                Self { base_url }
            }

            #(#methods)*
        }

        #(#hooks)*
    };

    TokenStream::from(expanded)
}
