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
                        format!("Unknown argument `{ident}`"),
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
#[allow(clippy::too_many_lines)]
pub fn fetch_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let module_name = format_ident!("{}_fetch_schema", enum_name.to_string().to_snake_case());
    let fetch_client_name = format_ident!("{}FetchClient", enum_name);
    let fetch_client_options_name = format_ident!("{}Options", fetch_client_name);
    let hook_options_name = format_ident!("{}HookOptions", enum_name);
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
        let variant_snake_case = variant_name.to_string().to_snake_case();
        let fetch_method_name = format_ident!("{}", variant_snake_case);
        let prepare_url_method_name = format_ident!("prepare_{}_url", variant_snake_case);
        let common_hook_name = format_ident!("use_common_{}", fetch_method_name);
        let hook_handle_name = format_ident!("Use{}Handle", variant_name);
        let hook_async_handle_name = format_ident!("Use{}AsyncHandle", variant_name);
        let hook_name = format_ident!("use_{}", fetch_method_name);
        let hook_with_options_name = format_ident!("{}_with_options", hook_name);
        let hook_name_async = format_ident!("{}_async", hook_name);
        let hook_with_options_name_async = format_ident!("{}_with_options_async", hook_name);

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
                    pub fn #prepare_url_method_name(&self) -> String {
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

                        if TypeId::of::<#query>() != TypeId::of::<()>() {
                            if url.contains('?') {
                                url.push('&');
                            } else {
                                url.push('?');
                            }
                        }

                        url
                    }

                    pub async fn #fetch_method_name(&self, url: String, params: #method_params_struct_name) -> Result<String, FetchError> {
                        let fetch_options = FetchOptions {
                            slugs: params.slugs,
                            query: params.query,
                            body: params.body,
                            middlewares: self.middlewares.clone(),
                        };

                        fetch::<#slugs, #query, #body>(
                            HttpMethod::from(#http_method),
                            url.as_str(),
                            fetch_options,
                        ).await
                    }
                });

                hooks.push(quote! {
                    #[derive(Clone, Debug)]
                    pub struct #hook_handle_name {
                        pub data: UseStateHandle<Option<#res>>,
                        pub loading: UseStateHandle<bool>,
                        pub error: UseStateHandle<Option<FetchError>>,
                    }

                    impl PartialEq for #hook_handle_name {
                        fn eq(&self, other: &Self) -> bool {
                            self.data == other.data
                                && self.loading == other.loading
                        }
                    }

                    #[derive(Clone, Debug)]
                    pub struct #hook_async_handle_name {
                        pub data: UseStateHandle<Option<#res>>,
                        pub loading: UseStateHandle<bool>,
                        pub error: UseStateHandle<Option<FetchError>>,
                        pub trigger: Callback<#method_params_struct_name>,
                    }

                    impl PartialEq for #hook_async_handle_name {
                        fn eq(&self, other: &Self) -> bool {
                            self.data == other.data
                                && self.loading == other.loading
                        }
                    }

                    #[hook]
                    fn #common_hook_name(options: Option<#hook_options_name>) -> #hook_async_handle_name {
                        let client = use_context::<#fetch_client_name>()
                            .expect(
                                &format!(
                                    "{} must be used within a {} provider",
                                    stringify!(#hook_name),
                                    stringify!(#fetch_client_context_provider_name)
                                )
                            );

                        let data = use_state(|| None::<#res>);
                        let loading = use_state(|| false);
                        let error = use_state(|| None::<FetchError>);

                        let trigger = use_callback(client.clone(), {
                            let data = data.clone();
                            let loading = loading.clone();
                            let error = error.clone();

                            move |params: #method_params_struct_name, client| {
                                let data = data.clone();
                                let loading = loading.clone();
                                let error = error.clone();
                                let client = client.clone();
                                let options = options.clone();

                                let cache_policy = {
                                    let custom_cache_policy = options.as_ref().and_then(
                                        |o| o.cache_options.as_ref().and_then(|options| options.policy.clone())
                                    );

                                    custom_cache_policy.unwrap_or_else(|| {
                                        let cache_ref = client.cache.borrow();
                                        cache_ref.policy().clone()
                                    })
                                };

                                let method = HttpMethod::from(#http_method);
                                let url = client.#prepare_url_method_name();

                                let (cache_key, cache_entry) = match cache_policy {
                                    CachePolicy::NetworkOnly => {
                                        (None, None)
                                    }
                                    _ => {
                                        if let Ok(cache_key) = generate_cache_key(&method, url.as_str(), &params.slugs, &params.query, &params.body) {
                                            let cache_entry = {
                                                let cache_ref = client.cache.borrow();
                                                cache_ref.get(&cache_key).cloned()
                                            };

                                            (Some(cache_key), cache_entry)
                                        } else {
                                            (None, None)
                                        }
                                    }
                                };

                                spawn_local(async move {
                                    loading.set(true);

                                    match cache_policy {
                                        CachePolicy::StaleWhileRevalidate => {
                                            if let Some(key) = cache_key {
                                                if let Some(entry) = cache_entry {
                                                    match deserialize_cached_data::<#res>(&entry.data) {
                                                        Ok(res) => {
                                                            data.set(Some(res));
                                                        }
                                                        Err(err) => {
                                                            error.set(Some(err));
                                                        }
                                                    }
                                                }

                                                match client.#fetch_method_name(url, params).await {
                                                    Ok(res) => {
                                                        match deserialize_response_and_store_cache::<#res>(
                                                            &res,
                                                            &client.cache,
                                                            &key,
                                                            options.as_ref().and_then(
                                                                |o| o.cache_options.as_ref().and_then(|options| options.max_age)
                                                            )
                                                        ) {
                                                            Ok(res) => {
                                                                data.set(Some(res));
                                                            }
                                                            Err(err) => {
                                                                error.set(Some(err));
                                                            }
                                                        }
                                                    }
                                                    Err(err) => {
                                                        error.set(Some(err));
                                                    }
                                                }
                                            }
                                        }
                                        CachePolicy::CacheThenNetwork => {
                                            if let Some(key) = cache_key {
                                                if let Some(entry) = cache_entry {
                                                    match deserialize_cached_data::<#res>(&entry.data) {
                                                        Ok(res) => {
                                                            data.set(Some(res));
                                                        }
                                                        Err(err) => {
                                                            error.set(Some(err));
                                                        }
                                                    }
                                                } else {
                                                    match client.#fetch_method_name(url, params).await {
                                                        Ok(res) => {
                                                            match deserialize_response_and_store_cache::<#res>(
                                                                &res,
                                                                &client.cache,
                                                                &key,
                                                                options.as_ref().and_then(
                                                                    |o| o.cache_options.as_ref().and_then(|options| options.max_age)
                                                                )
                                                            ) {
                                                                Ok(res) => {
                                                                    data.set(Some(res));
                                                                }
                                                                Err(err) => {
                                                                    error.set(Some(err));
                                                                }
                                                            }
                                                        }
                                                        Err(err) => {
                                                            error.set(Some(err));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        CachePolicy::NetworkOnly => {
                                            match client.#fetch_method_name(url, params).await {
                                                Ok(res) => {
                                                    match deserialize_response::<#res>(&res) {
                                                        Ok(res) => {
                                                            data.set(Some(res));
                                                        }
                                                        Err(err) => {
                                                            error.set(Some(err));
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    error.set(Some(err));
                                                }
                                            }
                                        }
                                        CachePolicy::CacheOnly => {
                                            if let Some(entry) = cache_entry {
                                                match deserialize_cached_data::<#res>(&entry.data) {
                                                    Ok(res) => {
                                                        data.set(Some(res));
                                                    }
                                                    Err(err) => {
                                                        error.set(Some(err));
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    loading.set(false);
                                });
                            }
                        });

                        #hook_async_handle_name {
                            data,
                            loading,
                            error,
                            trigger,
                        }

                    }

                    #[hook]
                    pub fn #hook_with_options_name_async(options: #hook_options_name) -> #hook_async_handle_name {
                        #common_hook_name(Some(options))
                    }

                    #[hook]
                    pub fn #hook_name_async() -> #hook_async_handle_name {
                        #common_hook_name(None)
                    }

                    #[hook]
                    pub fn #hook_name(params: #method_params_struct_name) -> #hook_handle_name {
                        let hook = #common_hook_name(None);

                        use_effect_with((params.clone(), hook.trigger.clone()), |(params, trigger)| {
                            trigger.emit(params.clone());
                        });

                        #hook_handle_name {
                            data: hook.data,
                            loading: hook.loading,
                            error: hook.error,
                        }
                    }

                    #[hook]
                    pub fn #hook_with_options_name(params: #method_params_struct_name, options: #hook_options_name) -> #hook_handle_name {
                        let hook = #common_hook_name(Some(options));

                        use_effect_with((params.clone(), hook.trigger.clone()), |(params, trigger)| {
                            trigger.emit(params.clone());
                        });

                        #hook_handle_name {
                            data: hook.data,
                            loading: hook.loading,
                            error: hook.error,
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
        mod #module_name {
            use crate::*;
            use yew::hook;
            use std::rc::Rc;
            use std::cell::RefCell;
            use std::any::TypeId;
            use std::borrow::BorrowMut;
            use yewlish_fetch_utils::{
                fetch, FetchOptions, HttpMethod, FetchError, Middleware, Cacheable, Cache, CacheOptions, CacheEntry,
                generate_cache_key, CachePolicy, deserialize_cached_data, deserialize_response, deserialize_response_and_store_cache
            };
            use yew::prelude::*;
            use yew::platform::spawn_local;

            #(#structs)*
            #(#errors)*

            #[derive(Clone, PartialEq, Default)]
            pub struct #hook_options_name {
                pub cache_options: Option<CacheOptions>,
            }

            #[derive(Clone)]
            pub struct #fetch_client_name {
                pub base_url: String,
                pub middlewares: Vec<Middleware>,
                pub cache: Rc<RefCell<dyn Cacheable>>,
                _marker: std::marker::PhantomData<#enum_name>,
            }

            impl PartialEq for #fetch_client_name {
                fn eq(&self, other: &Self) -> bool {
                    self.base_url == other.base_url
                    && self.middlewares.len() == other.middlewares.len()
                }
            }

            impl #fetch_client_name {
                pub fn new(base_url: &str) -> Self {
                    Self {
                        base_url: base_url.to_string(),
                        middlewares: Vec::new(),
                        cache: Rc::new(RefCell::new(Cache::default())),
                        _marker: std::marker::PhantomData
                    }
                }

                #(#methods)*
            }

            #[derive(Clone)]
            pub struct #fetch_client_options_name {
                pub middlewares: Vec<Middleware>,
                pub cache: Rc<RefCell<dyn Cacheable>>,
            }

            impl PartialEq for #fetch_client_options_name {
                fn eq(&self, other: &Self) -> bool {
                    self.middlewares.len() == other.middlewares.len()
                }
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
        }

        pub use #module_name::*;
    };

    TokenStream::from(expanded)
}
