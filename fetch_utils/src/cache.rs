use js_sys::Date;

#[derive(Default, Clone, PartialEq)]
pub enum CachePolicy {
    #[default]
    NetworkAndCache,
    NetworkOnly,
    CacheOnly,
}

#[derive(Default, Clone, PartialEq)]
pub struct CacheOptions {
    pub policy: Option<CachePolicy>,
    pub max_age: Option<f64>,
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
}

impl Default for Cache {
    fn default() -> Self {
        Self::new(CacheOptions::default())
    }
}

pub trait Cacheable {
    fn policy(&self) -> &CachePolicy;
    fn max_age(&self) -> f64;
    fn set(&mut self, key: &str, value: &serde_json::Value);
    fn get(&self, key: &str) -> Option<&CacheEntry>;
    fn remove(&mut self, key: &str);
    fn clear(&mut self);
}

const CACHE_MAX_AGE: f64 = 10.0 * 60.0 * 1000.0; // Ten minutes

impl Cache {
    fn new(options: CacheOptions) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            policy: options.policy.unwrap_or_default(),
            max_age: options.max_age.unwrap_or(CACHE_MAX_AGE),
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

    fn set(&mut self, key: &str, value: &serde_json::Value) {
        self.entries.insert(
            key.to_string(),
            CacheEntry {
                timestamp: Date::now(),
                data: value.clone(),
            },
        );
    }

    fn get(&self, key: &str) -> Option<&CacheEntry> {
        self.entries.get(key)
    }

    fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}
