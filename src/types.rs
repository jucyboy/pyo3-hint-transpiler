use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Transpiles {
    pub primitives: HashMap<String, String>,
    pub strings: HashMap<String, String>,
    pub collections: HashMap<String, String>,
    pub wrappers: HashMap<String, String>,
    pub r#async: HashMap<String, String>,
    pub pyo3: HashMap<String, String>,
    pub typing: HashMap<String, String>,
}

pub fn load() -> Transpiles {
    let raw = include_str!("../transpiles.json");
    serde_json::from_str(raw).unwrap()
}

impl Transpiles {
    pub fn resolve(&self, rust: &str) -> Option<&str> {
        self.primitives
            .get(rust)
            .or_else(|| self.strings.get(rust))
            .or_else(|| self.collections.get(rust))
            .or_else(|| self.wrappers.get(rust))
            .or_else(|| self.r#async.get(rust))
            .or_else(|| self.pyo3.get(rust))
            .or_else(|| self.typing.get(rust))
            .map(|s| s.as_str())
    }
}

// dummy usage to silence dead_code warnings
#[allow(dead_code)]
pub fn _use_transpiles() {
    let t = load();
    t.primitives.get("i32");
    t.strings.get("str");
    t.collections.get("Vec");
    t.wrappers.get("Option");
    t.r#async.get("Future");
    t.pyo3.get("PyResult");
    t.typing.get("Any");
    let _ = t.resolve("i32");
}
