use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result as SynResult},
    parse_macro_input, Attribute, Data, DeriveInput, Ident, LitStr, Token, Type,
};

struct FetchAttributeArgs {
    path: LitStr,
    slugs: Option<Type>,
    query: Option<Type>,
    body: Option<Type>,
    res: Option<Type>,
}

impl Parse for FetchAttributeArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let path: LitStr = input.parse()?;
        let mut slugs = None;
        let mut query = None;
        let mut body = None;
        let mut res = None;

        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match ident.to_string().as_str() {
                "slugs" => {
                    let ty: Type = input.parse()?;
                    slugs = Some(ty);
                }
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
            slugs,
            query,
            body,
            res,
        })
    }
}

static HTTP_VERBS: [&str; 5] = ["GET", "POST", "PUT", "PATCH", "DELETE"];

fn extract_attrs(attrs: &[Attribute]) -> SynResult<(String, String, Type, Type, Type, Type)> {
    for attr in attrs {
        if let Some(ident) = attr.path().get_ident() {
            if HTTP_VERBS.contains(&ident.to_string().to_uppercase().as_str()) {
                let FetchAttributeArgs {
                    path,
                    slugs,
                    query,
                    body,
                    res,
                } = attr.parse_args()?;

                let method = ident.to_string().to_uppercase();
                let path = path.value();
                let slugs = slugs.unwrap_or_else(|| syn::parse_quote! { () });
                let query = query.unwrap_or_else(|| syn::parse_quote! { () });
                let body = body.unwrap_or_else(|| syn::parse_quote! { () });
                let res = res.unwrap_or_else(|| syn::parse_quote! { () });

                return Ok((method, path, slugs, query, body, res));
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
    let fetch_client_context_props_name = format_ident!("{}ProviderProps", fetch_client_name);
    let fetch_client_context_provider_name = format_ident!("{}Provider", fetch_client_name);
    let fetch_client_context_snake_case_provider_name = format_ident!(
        "{}",
        fetch_client_context_provider_name
            .to_string()
            .to_snake_case()
    );

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
    let mut variant_names = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;
        variant_names.push(variant_name);

        let method_params_struct_name = format_ident!("{}Params", variant_name);
        let method_name = format_ident!("{}", variant_name.to_string().to_snake_case());
        let hook_handle_name = format_ident!("Use{}Handle", variant_name);
        let hook_name = format_ident!("use_{}", method_name);

        match extract_attrs(&variant.attrs) {
            Ok((http_method, path, slugs, query, body, res)) => {
                structs.push(quote! {
                    #[derive(Default, Clone, PartialEq)]
                    pub struct #method_params_struct_name {
                        pub slugs: #slugs,
                        pub query: #query,
                        pub body: #body,
                    }
                });

                methods.push(quote! {
                    pub async fn #method_name(&self, params: #method_params_struct_name) -> Result<#res, FetchError> {
                        let path = if #path.starts_with('/') {
                            &#path[1..]
                        } else {
                            &#path
                        };

                        let base_url = if self.base_url.ends_with('/') {
                            &self.base_url[..self.base_url.len() - 1]
                        } else {
                            &self.base_url
                        };

                        let mut url = format!("{}/{}", base_url, path);

                        if #query != () {
                            if url.contains('?') {
                                url.push('&');
                            } else {
                                url.push('?');
                            }
                        }

                        fetch::<#slugs, #query, #body, #res>(
                            HttpMethod::from(#http_method),
                            url.as_str(),
                            params.slugs,
                            params.query,
                            params.body,
                            Vec::new(),
                        ).await
                    }
                });

                hooks.push(quote! {
                    #[derive(Clone, Debug)]
                    pub struct #hook_handle_name {
                        pub data: UseStateHandle<Option<#res>>,
                        pub error: UseStateHandle<Option<FetchError>>,
                        pub loading: UseStateHandle<bool>,
                    }

                    impl PartialEq for #hook_handle_name {
                        fn eq(&self, other: &Self) -> bool {
                            self.data == other.data
                                && self.loading == other.loading
                        }
                    }

                    #[hook]
                    pub fn #hook_name(params: #method_params_struct_name) -> #hook_handle_name {
                        let client = use_context::<#fetch_client_name>()
                            .expect(&format!("{} must be used within a {} provider", stringify!(#hook_name), stringify!(#fetch_client_context_provider_name)));

                        let data = use_state(|| None::<#res>);
                        let error = use_state(|| None::<FetchError>);
                        let loading = use_state(|| false);

                        let trigger = use_callback(client.clone(), {
                            let data = data.clone();
                            let error = error.clone();
                            let loading = loading.clone();

                            move |params: #method_params_struct_name, client| {
                                let data = data.clone();
                                let error = error.clone();
                                let loading = loading.clone();
                                let client = client.clone();

                                spawn_local(async move {
                                    loading.set(true);

                                    match client.#method_name(params).await {
                                        Ok(res) => {
                                            data.set(Some(res));
                                        }
                                        Err(err) => {
                                            error.set(Some(err));
                                        }
                                    }

                                    loading.set(false);
                                });
                            }
                        });

                        use_effect_with((params.clone(), trigger.clone()), |(params, trigger)| {
                            trigger.emit(params.clone());
                        });

                        #hook_handle_name {
                            data,
                            error,
                            loading,
                        }
                    }
                });
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    let variant_references = quote! {
        #[allow(dead_code)]
        fn _use_variants() {
            // Reference each variant to prevent unused code warnings
            #(
                let _ = &#enum_name::#variant_names;
            )*
        }

        #[allow(dead_code)]
        const _: fn() = _use_variants;
    };

    let errors = errors.into_iter().map(|error| error.to_compile_error());

    let expanded = quote! {
        use yew::hook;
        use yewlish_fetch_utils::{fetch, HttpMethod, FetchError};
        use yew::platform::spawn_local;

        #(#structs)*
        #(#errors)*

        #[derive(Clone, PartialEq)]
        pub struct #fetch_client_name {
            pub base_url: String,
            _marker: std::marker::PhantomData<#enum_name>,
        }

        impl #fetch_client_name {
            pub fn new(base_url: String) -> Self {
                Self { base_url, _marker: std::marker::PhantomData }
            }

            #(#methods)*
        }

        #[derive(Clone, PartialEq, Properties)]
        pub struct #fetch_client_context_props_name {
            pub client: #fetch_client_name,
            pub children: Children,
        }

        #[function_component(#fetch_client_context_provider_name)]
        pub fn #fetch_client_context_snake_case_provider_name(props: &#fetch_client_context_props_name) -> Html {
            html! {
                <ContextProvider<#fetch_client_name> context={props.client.clone()}>
                    {for props.children.iter()}
                </ContextProvider<#fetch_client_name>>
            }
        }

        #(#hooks)*
        #variant_references
    };

    TokenStream::from(expanded)
}
