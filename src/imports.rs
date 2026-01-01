use std::collections::{BTreeMap, BTreeSet};

pub struct Imports {
    map: BTreeMap<String, BTreeSet<String>>,
}

impl Imports {
    pub fn new() -> Self {
        Self { map: BTreeMap::new() }
    }

    pub fn register(&mut self, dotted: &str) -> String {
        if !dotted.contains('.') {
            return dotted.to_string();
        }

        let parts: Vec<&str> = dotted.split('.').collect();
        let name = parts.last().unwrap().to_string();
        let module = parts[..parts.len() - 1].join(".");

        self.map.entry(module).or_default().insert(name.clone());
        name
    }

    pub fn render(&self) -> String {
        self.map
            .iter()
            .map(|(m, s)| {
                format!(
                    "from {} import {}",
                    m,
                    s.iter().cloned().collect::<Vec<_>>().join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// minimal usage to kill warnings
#[allow(dead_code)]
pub fn dummy_usage() {
    let mut imports = Imports::new();
    imports.register("typing.Any");
    imports.register("pyo3.PyResult");
    let _ = imports.render();
}
