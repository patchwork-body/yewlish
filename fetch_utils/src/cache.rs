use js_sys::Date;
use std::collections::hash_map::Iter;
use wasm_bindgen::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub enum CachePolicy {
    #[default]
    StaleWhileRevalidate,
    CacheThenNetwork,
    NetworkOnly,
    CacheOnly,
}

#[derive(Default, Clone, PartialEq)]
pub struct CacheOptions {
    pub policy: Option<CachePolicy>,
    pub max_age: Option<f64>,
    pub cleanup_interval: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CacheEntry {
    pub timestamp: f64,
    pub data: serde_json::Value,
}

pub struct Cache {
    entries: std::collections::HashMap<String, CacheEntry>,
    policy: CachePolicy,
    max_age: f64,
    cleanup_handle: Option<i32>,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new(CacheOptions::default())
    }
}

pub trait Cacheable {
    fn policy(&self) -> &CachePolicy;
    fn max_age(&self) -> f64;
    fn set(&mut self, key: &str, value: &serde_json::Value, max_age: Option<f64>);
    fn get(&self, key: &str) -> Option<&CacheEntry>;
    fn iter(&self) -> Iter<String, CacheEntry>;
    fn remove(&mut self, key: &str);
    fn clear(&mut self);
}

const CACHE_MAX_AGE: f64 = 10.0 * 60.0 * 1000.0; // Ten minutes
const DEFAULT_CLEANUP_INTERVAL: f64 = 60.0 * 1000.0; // One minute

impl Cache {
    fn new(options: CacheOptions) -> Self {
        let mut cache = Self {
            entries: std::collections::HashMap::new(),
            policy: options.policy.unwrap_or_default(),
            max_age: options.max_age.unwrap_or(CACHE_MAX_AGE),
            cleanup_handle: None,
        };

        let interval = options.cleanup_interval.unwrap_or(DEFAULT_CLEANUP_INTERVAL);
        cache.start_cleanup(interval as i32);

        cache
    }

    fn start_cleanup(&mut self, interval_ms: i32) {
        let window = web_sys::window().expect("no global window exists");
        let this = self as *mut Cache;

        let closure = Closure::wrap(Box::new(move || {
            let cache = unsafe { &mut *this };
            cache.cleanup();
        }) as Box<dyn FnMut()>);

        let handle = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                interval_ms,
            )
            .expect("failed to set interval");

        closure.forget();

        self.cleanup_handle = Some(handle);
    }

    fn cleanup(&mut self) {
        self.entries
            .retain(|_, entry| entry.timestamp > Date::now());
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        if let Some(handle) = self.cleanup_handle {
            if let Some(window) = web_sys::window() {
                window.clear_interval_with_handle(handle);
            }
        }
    }
}

impl Cacheable for Cache {
    fn policy(&self) -> &CachePolicy {
        &self.policy
    }

    fn max_age(&self) -> f64 {
        self.max_age
    }

    fn set(&mut self, key: &str, value: &serde_json::Value, max_age: Option<f64>) {
        self.entries.insert(
            key.to_string(),
            CacheEntry {
                timestamp: Date::now() + max_age.unwrap_or(self.max_age),
                data: value.clone(),
            },
        );
    }

    fn get(&self, key: &str) -> Option<&CacheEntry> {
        if let Some(cache_entry) = self.entries.get(key) {
            if cache_entry.timestamp > js_sys::Date::now() {
                return Some(cache_entry);
            }
        }

        None
    }

    #[must_use]
    fn iter(&self) -> Iter<String, CacheEntry> {
        self.entries.iter()
    }

    fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}
