use std::collections::HashMap;

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

static ALLOWED_METHODS: [&str; 6] = ["GET", "POST", "PUT", "PATCH", "DELETE", "WS"];

fn extract_attrs(attrs: &[Attribute]) -> SynResult<(String, String, Type, Type, Type, Type)> {
    for attr in attrs {
        if let Some(ident) = attr.path().get_ident() {
            if ALLOWED_METHODS.contains(&ident.to_string().to_uppercase().as_str()) {
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

#[proc_macro_derive(FetchSchema, attributes(get, post, put, patch, delete, ws))]
#[allow(clippy::too_many_lines)]
pub fn fetch_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let state_enum_name = format_ident!("{}State", enum_name);
    let data_enum_name = format_ident!("{}WsData", enum_name);
    let ws_state_struct_name = format_ident!("{}WsState", enum_name);
    let module_name = format_ident!("{}_fetch_schema", enum_name.to_string().to_snake_case());
    let fetch_client_name = format_ident!("{}FetchClient", enum_name);
    let fetch_client_options_name = format_ident!("{}Options", fetch_client_name);
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

    let mut structs = Vec::new();
    let mut methods = Vec::new();
    let mut hooks = Vec::new();
    let mut errors = Vec::new();
    let mut variant_names = Vec::new();
    let mut state_enum_variants = Vec::new();
    let mut data_enum_variants = Vec::new();

    let mut merged_data_enum_variants = HashMap::new();

    for variant in variants {
        match extract_attrs(&variant.attrs) {
            Ok((verb, _path, _slugs, _query, _body, res)) => {
                if verb == "WS" {
                    let res_string = quote!(#res).to_string();
                    let variant_name = &variant.ident;

                    merged_data_enum_variants
                        .entry(res_string)
                        .or_insert(variant_name);
                }
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    for variant in variants {
        let variant_name = &variant.ident;
        variant_names.push(variant_name);

        let state_struct_name = format_ident!("{}State", variant_name);
        let method_params_struct_name = format_ident!("{}Params", variant_name);
        let method_open_params_struct_name = format_ident!("{}HookOpenParams", variant_name);
        let hook_open_params_struct_name = format_ident!("{}OpenParams", variant_name);
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
        let hook_options_name = format_ident!("{}Options", variant_name);

        match extract_attrs(&variant.attrs) {
            Ok((verb, path, slugs, query, body, res)) => {
                // Structs for hooks and methods
                if verb == "WS" {
                    structs.push(quote! {
                        #[derive(Default, Clone, PartialEq)]
                        pub struct #hook_open_params_struct_name {
                            pub slugs: #slugs,
                            pub query: #query,
                        }

                        #[derive(Default, Clone, PartialEq)]
                        struct #method_open_params_struct_name {
                            pub slugs: #slugs,
                            pub query: #query,
                            pub onopen: Option<Callback<web_sys::Event>>,
                            pub onmessage: Option<Callback<web_sys::MessageEvent>>,
                            pub onerror: Option<Callback<web_sys::ErrorEvent>>,
                            pub onclose: Option<Callback<web_sys::CloseEvent>>,
                        }

                        #[derive(Clone, Debug)]
                        pub struct #hook_handle_name {
                            pub data: UseStateHandle<Option<#res>>,
                            pub status: UseStateHandle<WsStatus>,
                            pub error: UseStateHandle<Option<FetchError>>,
                            pub send: Callback<#body>,
                            pub close: Callback<()>,
                        }

                        impl PartialEq for #hook_handle_name {
                            fn eq(&self, other: &Self) -> bool {
                                self.data == other.data
                                    && self.status == other.status
                            }
                        }

                        #[derive(Clone, Debug)]
                        pub struct #hook_async_handle_name {
                            pub data: UseStateHandle<Option<#res>>,
                            pub status: UseStateHandle<WsStatus>,
                            pub error: UseStateHandle<Option<FetchError>>,
                            pub send: Callback<#body>,
                            pub open: Callback<#hook_open_params_struct_name>,
                            pub close: Callback<()>,
                        }

                        impl PartialEq for #hook_async_handle_name {
                            fn eq(&self, other: &Self) -> bool {
                                self.data == other.data
                                    && self.status == other.status
                            }
                        }

                        #[derive(Clone, PartialEq, Default)]
                        pub struct #hook_options_name {
                            pub on_message: Option<Callback<web_sys::MessageEvent>>,
                            pub on_data: Option<Callback<#res, Option<#res>>>,
                            pub on_status_change: Option<Callback<WsStatus>>,
                            pub on_error: Option<Callback<FetchError>>,
                            pub on_open: Option<Callback<web_sys::Event>>,
                            pub on_close: Option<Callback<web_sys::CloseEvent>>,
                        }
                    });
                } else {
                    structs.push(quote! {
                        #[derive(Default, Clone, PartialEq)]
                        pub struct #method_params_struct_name {
                            pub slugs: #slugs,
                            pub query: #query,
                            pub body: #body,
                        }

                        #[derive(Clone, Debug, PartialEq)]
                        pub struct #state_struct_name {
                            pub data: UseStateHandle<Option<#res>>,
                            pub loading: UseStateHandle<bool>,
                            pub error: UseStateHandle<Option<FetchError>>,
                        }

                        #[derive(Clone, Debug)]
                        pub struct #hook_handle_name {
                            pub data: UseStateHandle<Option<#res>>,
                            pub loading: UseStateHandle<bool>,
                            pub error: UseStateHandle<Option<FetchError>>,
                            pub cancel: Callback<()>,
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
                            pub cancel: Callback<()>,
                        }

                        impl PartialEq for #hook_async_handle_name {
                            fn eq(&self, other: &Self) -> bool {
                                self.data == other.data
                                    && self.loading == other.loading
                            }
                        }

                        #[derive(Clone, PartialEq, Default)]
                        pub struct #hook_options_name {
                            pub cache_options: Option<CacheOptions>,
                            pub on_success: Option<Callback<#res>>,
                            pub on_error: Option<Callback<FetchError>>,
                        }
                    });
                }

                // Common Prepare URL method
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
                });

                // Non-WS methods
                if verb != "WS" {
                    state_enum_variants.push(quote! {
                        #variant_name(#state_struct_name),
                    });

                    methods.push(quote! {
                        pub async fn #fetch_method_name(&self, url: String, abort_signal: Rc<web_sys::AbortSignal>, params: #method_params_struct_name) -> Result<String, FetchError> {
                            let fetch_options = FetchOptions {
                                slugs: params.slugs,
                                query: params.query,
                                body: params.body,
                                middlewares: self.middlewares.as_slice(),
                                abort_signal: abort_signal.clone(),
                            };

                            fetch::<#slugs, #query, #body>(
                                HttpMethod::from(#verb),
                                url.as_str(),
                                fetch_options,
                            ).await
                        }
                    });
                }

                // Hooks
                if verb == "WS" {
                    let Some(variant_name) =
                        merged_data_enum_variants.get(&quote!(#res).to_string())
                    else {
                        return TokenStream::from(quote! {
                            compile_error!("Variant name not found");
                        });
                    };

                    if !data_enum_variants
                        .iter()
                        .any(|v| quote!(#v).to_string().contains(&variant_name.to_string()))
                    {
                        data_enum_variants.push(quote! {
                            #variant_name(#res),
                        });
                    }

                    hooks.push(quote! {
                        #[hook]
                        fn #common_hook_name(options: Option<#hook_options_name>) -> #hook_async_handle_name {
                            let client = use_context::<Rc<#fetch_client_name>>()
                                .expect(
                                    &format!(
                                        "{} must be used within a {} provider",
                                        stringify!(#hook_name),
                                        stringify!(#fetch_client_context_provider_name)
                                    )
                                );

                            let data = use_state(|| None::<#res>);
                            let status = use_state(|| WsStatus::Closed);
                            let error = use_state(|| None::<FetchError>);
                            let state_key_ref = use_mut_ref(|| None::<String>);
                            let slot_key_ref = use_mut_ref(|| None::<usize>);

                            let onopen = use_callback(options.clone(), {
                                let status = status.clone();

                                move |event: web_sys::Event, options| {
                                    if let Some(on_open) = options.as_ref().and_then(|o| o.on_open.clone()) {
                                        on_open.emit(event.clone());
                                    }

                                    status.set(WsStatus::Open);
                                }
                            });

                            let onmessage = use_callback(options.clone(), {
                                let data = data.clone();
                                let error = error.clone();

                                move |(event, res): (web_sys::MessageEvent, #data_enum_name), options| {
                                    if let Some(on_message) = options.as_ref().and_then(|o| o.on_message.clone()) {
                                        on_message.emit(event.clone());
                                    }

                                    if let #data_enum_name::#variant_name(res) = res {
                                        if let Some(on_data) = options.as_ref().and_then(|o| o.on_data.clone()) {
                                            if let Some(res) = on_data.emit(res.clone()) {
                                                data.set(Some(res));
                                            }
                                        } else {
                                            data.set(Some(res));
                                        }
                                    }
                                }
                            });

                            let onerror = use_callback(options.clone(), {
                                let error = error.clone();

                                move |err: FetchError, options| {
                                    if let Some(on_error) = options.as_ref().and_then(|o| o.on_error.clone()) {
                                        on_error.emit(err.clone());
                                    }

                                    error.set(Some(err));
                                }
                            });

                            let onclose = use_callback(options.clone(), {
                                let status = status.clone();

                                move |event: web_sys::CloseEvent, options| {
                                    if let Some(on_close) = options.as_ref().and_then(|o| o.on_close.clone()) {
                                        on_close.emit(event.clone());
                                    }

                                    status.set(WsStatus::Closed);
                                }
                            });

                            let subscriber = use_memo(
                                (onopen.clone(), onmessage.clone(), onerror.clone(), onclose.clone()),
                                |(onopen, onmessage, onerror, onclose)| WebSocketSubscriber {
                                    onopen: onopen.clone(),
                                    onmessage: onmessage.clone(),
                                    onerror: onerror.clone(),
                                    onclose: onclose.clone(),
                                }
                            );

                            let open = use_callback((client.clone(), subscriber.clone()), {
                                let error = error.clone();
                                let status = status.clone();
                                let state_key_ref = state_key_ref.clone();
                                let slot_key_ref = slot_key_ref.clone();

                                move |params: #hook_open_params_struct_name, (client, subscriber)| {
                                    status.set(WsStatus::Opening);
                                    let url = client.#prepare_url_method_name();

                                    let Ok(state_key) = generate_state_key("ws", url.as_str(), &params.slugs, &params.query) else {
                                        error.set(Some(FetchError::UnknownError("Failed to generate state key".to_string())));
                                        return;
                                    };

                                    state_key_ref.replace(Some(state_key.clone()));
                                    let mut queries = (*client.queries).borrow_mut();

                                    if let Some(mut slotmap) = queries.get_mut(&state_key) {
                                        if slot_key_ref.borrow().is_some() {
                                            return;
                                        }

                                        if let Some(#state_enum_name::Ws(state)) = slotmap.first().cloned() {
                                            match (*state.web_socket_watcher).borrow_mut().subscribe((**subscriber).clone()) {
                                                Ok(()) => {
                                                    let slot_key = slotmap.insert(#state_enum_name::Ws(#ws_state_struct_name {
                                                        web_socket_watcher: state.web_socket_watcher.clone(),
                                                    }));

                                                    slot_key_ref.replace(Some(slot_key));
                                                    error.set(None);
                                                }
                                                Err(err) => {
                                                    error.set(Some(err));
                                                }
                                            }
                                        }
                                    } else {
                                        match build_url(url.as_str(), &params.slugs, &params.query) {
                                            Ok(url) => {
                                                let mut slotmap = SlotMap::<#state_enum_name>::new();
                                                let mut watcher = WebSocketWatcher::<#data_enum_name>::new(String::from(url.to_string()));

                                                match watcher.subscribe((**subscriber).clone()) {
                                                    Ok(()) => {
                                                        error.set(None);
                                                    }
                                                    Err(err) => {
                                                        error.set(Some(err));
                                                        return;
                                                    }
                                                }

                                                let slot_key = slotmap.insert(#state_enum_name::Ws(#ws_state_struct_name {
                                                    web_socket_watcher: Rc::new(RefCell::new(watcher)),
                                                }));

                                                slot_key_ref.replace(Some(slot_key));
                                                queries.insert(state_key, slotmap);
                                            },
                                            Err(err) => {
                                                error.set(Some(err));
                                            }
                                        }
                                    }
                                }
                            });

                            let send = use_callback((client.clone(), state_key_ref.clone(), slot_key_ref.clone()), {
                                let error = error.clone();

                                move |message: #body, (client, state_key_ref, slot_key_ref)| {
                                    let (Some(state_key), Some(slot_key)) = (state_key_ref.borrow().as_ref().cloned(), slot_key_ref.borrow().as_ref().cloned()) else {
                                        error.set(Some(FetchError::UnknownError(
                                            format!("State key or slot key is missing. state key: {state_key_ref:?}, slot key: {slot_key_ref:?}")
                                        )));

                                        return;
                                    };

                                    if let Some(slotmap) = client.queries.borrow().get(&state_key) {
                                        if let Some(#state_enum_name::Ws(state)) = slotmap.get(slot_key) {
                                            match state.web_socket_watcher.borrow().send(&message) {
                                                Ok(()) => {
                                                    error.set(None);
                                                }
                                                Err(err) => {
                                                    error.set(Some(err));
                                                }
                                            }
                                        }
                                    }
                                }
                            });

                            let close = use_callback((client.clone(), subscriber.clone(), state_key_ref.clone(), slot_key_ref.clone()), {
                                let status = status.clone();
                                let error = error.clone();

                                move |(), (client, subscriber, state_key_ref, slot_key_ref)| {
                                    status.set(WsStatus::Closing);

                                    let (Some(state_key), Some(slot_key)) = (state_key_ref.borrow().as_ref().cloned(), slot_key_ref.borrow().as_ref().cloned()) else {
                                        error.set(Some(FetchError::UnknownError("State key or slot key is missing".to_string())));
                                        return;
                                    };

                                    if let Some(slotmap) = client.queries.borrow().get(&state_key) {
                                        if let Some(#state_enum_name::Ws(state)) = slotmap.get(slot_key) {
                                            match (*state.web_socket_watcher).borrow_mut().unsubscribe(&*subscriber.as_ref()) {
                                                Ok(()) => {}
                                                Err(err) => {
                                                    error.set(Some(err));
                                                }
                                            }
                                        }
                                    }
                                }
                            });

                            use_effect_with((status.clone(), options.clone()), |(status, options)| {
                                let on_status_change = options.as_ref().and_then(|o| o.on_status_change.clone()).unwrap_or_else(Callback::noop);
                                on_status_change.emit((**status).clone());
                            });

                            use_effect_with((client.clone(), close.clone(), state_key_ref.clone(), slot_key_ref.clone()), {
                                let error = error.clone();

                                move |(client, close, state_key_ref, slot_key_ref)| {
                                    let client = client.clone();
                                    let close = close.clone();
                                    let state_key_ref = state_key_ref.clone();
                                    let slot_key_ref = slot_key_ref.clone();

                                    move || {
                                        close.emit(());

                                        let (Some(state_key), Some(slot_key)) = (state_key_ref.borrow().as_ref().cloned(), slot_key_ref.borrow().as_ref().cloned()) else {
                                            error.set(Some(FetchError::UnknownError("State key or slot key is missing".to_string())));
                                            return;
                                        };

                                        let mut queries = (*client.queries).borrow_mut();

                                        if let Some(slotmap) = queries.get_mut(&state_key) {
                                            slotmap.remove(slot_key);

                                            if slotmap.is_empty() {
                                                queries.remove(&state_key);
                                            }
                                        }
                                    }
                                }
                            });

                            #hook_async_handle_name {
                                data,
                                status,
                                error,
                                send,
                                open,
                                close,
                            }
                        }

                        #[hook]
                        pub fn #hook_name_async() -> #hook_async_handle_name {
                            #common_hook_name(None)
                        }

                        #[hook]
                        pub fn #hook_name(params: #hook_open_params_struct_name) -> #hook_handle_name {
                            let hook = #common_hook_name(None);

                            use_effect_with((params.clone(), hook.open.clone(), hook.close.clone()), |(params, open, close)| {
                                open.emit(params.clone());
                                let close = close.clone();

                                move || {
                                    close.emit(());
                                }
                            });

                            #hook_handle_name {
                                data: hook.data,
                                status: hook.status,
                                error: hook.error,
                                send: hook.send.clone(),
                                close: hook.close.clone(),
                            }
                        }

                        #[hook]
                        pub fn #hook_with_options_name_async(options: #hook_options_name) -> #hook_async_handle_name {
                            #common_hook_name(Some(options))
                        }

                        #[hook]
                        pub fn #hook_with_options_name(params: #hook_open_params_struct_name, options: #hook_options_name) -> #hook_handle_name {
                            let hook = #common_hook_name(Some(options));

                            use_effect_with((params.clone(), hook.open.clone(), hook.close.clone()), |(params, open, close)| {
                                open.emit(params.clone());
                                let close = close.clone();

                                move || {
                                    close.emit(());
                                }
                            });

                            #hook_handle_name {
                                data: hook.data,
                                status: hook.status,
                                error: hook.error,
                                send: hook.send.clone(),
                                close: hook.close.clone(),
                            }
                        }
                    });
                } else {
                    hooks.push(quote! {
                        #[hook]
                        fn #common_hook_name(options: Option<#hook_options_name>) -> #hook_async_handle_name {
                            let client = use_context::<Rc<#fetch_client_name>>()
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

                            let abort_controller = match web_sys::AbortController::new() {
                                Ok(controller) => Rc::new(controller),
                                Err(abort_controller_error) => {
                                    error.set(Some(FetchError::UnknownError(format!("{abort_controller_error:?}"))));

                                    return #hook_async_handle_name {
                                        data,
                                        loading,
                                        error,
                                        trigger: Callback::noop(),
                                        cancel: Callback::noop(),
                                    };
                                }
                            };

                            let abort_signal = Rc::new(abort_controller.signal());

                            let trigger = use_callback((client.clone(), options.clone()), {
                                let data = data.clone();
                                let loading = loading.clone();
                                let error = error.clone();
                                let abort_signal = abort_signal.clone();

                                move |params: #method_params_struct_name, (client, options)| {
                                    let data = data.clone();
                                    let loading = loading.clone();
                                    let error = error.clone();
                                    let client = client.clone();
                                    let options = options.clone();
                                    let abort_signal = abort_signal.clone();

                                    let cache_policy = {
                                        let custom_cache_policy = options.as_ref().and_then(
                                            |o| o.cache_options.as_ref().and_then(|options| options.policy.clone())
                                        );

                                        custom_cache_policy.unwrap_or_else(|| {
                                            let cache_ref = client.cache.borrow();
                                            cache_ref.policy().clone()
                                        })
                                    };

                                    let method = HttpMethod::from(#verb);
                                    let url = client.#prepare_url_method_name();

                                    let Ok(cache_key) = generate_cache_key(
                                        &method, url.as_str(), &params.slugs, &params.query, &params.body
                                    ) else {
                                        error.set(Some(FetchError::UnknownError("Failed to generate cache key".to_string())));
                                        return;
                                    };

                                    let cache_entry = {
                                        let cache_ref = client.cache.borrow();
                                        cache_ref.get(&cache_key).cloned()
                                    };

                                    spawn_local(async move {
                                        loading.set(true);

                                        match cache_policy {
                                            CachePolicy::StaleWhileRevalidate => {
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

                                                    match client.#fetch_method_name(url, abort_signal, params).await {
                                                        Ok(res) => {
                                                            match deserialize_response_and_store_cache::<#res>(
                                                                &res,
                                                                &client.cache,
                                                                &cache_key,
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
                                            CachePolicy::CacheThenNetwork => {
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
                                                        match client.#fetch_method_name(url, abort_signal, params).await {
                                                            Ok(res) => {
                                                                match deserialize_response_and_store_cache::<#res>(
                                                                    &res,
                                                                    &client.cache,
                                                                    &cache_key,
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
                                            CachePolicy::NetworkOnly => {
                                                    match client.#fetch_method_name(url, abort_signal, params).await {
                                                        Ok(res) => {
                                                            match deserialize_response_and_store_cache::<#res>(
                                                                &res,
                                                                &client.cache,
                                                                &cache_key,
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

                            use_effect_with((error.clone(), options.clone()), |(error, options)| {
                                if let Some(error) = (**error).as_ref() {
                                    let on_error = options.as_ref().and_then(|o| o.on_error.clone()).unwrap_or_else(Callback::noop);
                                    on_error.emit(error.clone());
                                }
                            });

                            use_effect_with((data.clone(), options.clone()), |(data, options)| {
                                if let Some(data) = (**data).as_ref() {
                                    let on_success = options.as_ref().and_then(|o| o.on_success.clone()).unwrap_or_else(Callback::noop);
                                    on_success.emit(data.clone());
                                }
                            });

                            let cancel = use_callback(abort_controller.clone(), |(), controller| {
                                controller.abort();
                            });

                            #hook_async_handle_name {
                                data,
                                loading,
                                error,
                                trigger,
                                cancel,
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
                                cancel: hook.cancel,
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
                                cancel: hook.cancel,
                            }
                        }
                    });
                }
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
            use yewlish_fetch_utils::*;
            use yew::prelude::*;
            use yew::platform::spawn_local;
            use std::collections::HashMap;
            use std::any::Any;
            use wasm_bindgen::JsCast;

            #(#structs)*
            #(#errors)*

            #[derive(Clone, Debug, PartialEq, Deserialize)]
            #[serde(untagged)]
            enum #data_enum_name {
                #(#data_enum_variants)*
            }

            #[derive(Clone, Debug, PartialEq)]
            struct #ws_state_struct_name {
                pub web_socket_watcher: Rc<RefCell<WebSocketWatcher<#data_enum_name>>>,
            }

            #[derive(Clone, Debug, PartialEq)]
            enum #state_enum_name {
                #(#state_enum_variants)*
                Ws(#ws_state_struct_name),
            }

            #[derive(Clone)]
            pub struct #fetch_client_name {
                pub base_url: String,
                pub middlewares: Vec<Middleware>,
                pub cache: Rc<RefCell<dyn Cacheable>>,
                pub queries: Rc<RefCell<HashMap<String, SlotMap<#state_enum_name>>>>,
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
                        queries: Rc::new(RefCell::new(HashMap::new())),
                        _marker: std::marker::PhantomData
                    }
                }

                pub fn with_middlewares(mut self, middlewares: Vec<Middleware>) -> Self {
                    self.middlewares = middlewares;
                    self
                }

                pub fn with_cache(mut self, cache: Rc<RefCell<dyn Cacheable>>) -> Self {
                    self.cache = cache;
                    self
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

            #[hook]
            pub fn use_fetch_client() -> #fetch_client_name {
                use_context::<#fetch_client_name>()
                    .expect(
                        &format!(
                            "{} must be used within a {} provider",
                            stringify!(use_fetch_client),
                            stringify!(#fetch_client_context_provider_name)
                        )
                    )
            }

            #[derive(Clone, PartialEq, Properties)]
            pub struct #fetch_client_context_props_name {
                pub client: #fetch_client_name,
                pub children: Children,
            }

            #[function_component(#fetch_client_context_provider_name)]
            pub fn #fetch_client_context_snake_case_provider_name(props: &#fetch_client_context_props_name) -> Html {
                html! {
                    <ContextProvider<Rc<#fetch_client_name>> context={Rc::new(props.client.clone())}>
                        {for props.children.iter()}
                    </ContextProvider<Rc<#fetch_client_name>>>
                }
            }

            #(#hooks)*
            #variant_references
        }

        pub use #module_name::*;
    };

    TokenStream::from(expanded)
}
