// This code is just an example and not a part of the
// whole core of the tool.

pub fn hello() -> &'static str {
    "Hello from dummy lib!"
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub struct Dummy {
    pub value: i32,
}

impl Dummy {
    pub fn new(val: i32) -> Self {
        Self { value: val }
    }

    pub fn double(&self) -> i32 {
        self.value * 2
    }
}
