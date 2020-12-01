use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Env {
    Dev,
    Prod,
    Test,
}

impl Env {
    pub fn to_str(&self) -> &'static str {
        match self {
            Env::Dev => "development",
            Env::Prod => "production",
            Env::Test => "test",
        }
    }
}

#[derive(Clone)]
pub struct EnvData(HashMap<String, String>);

impl EnvData {
    pub fn new(data: HashMap<String, String>) -> Self {
        Self(data)
    }

    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    pub fn from_vec<K: ToString, V: ToString>(kvs: Vec<(K, V)>) -> Self {
        let mut data = HashMap::with_capacity(kvs.len());
        for (k, v) in kvs {
            data.insert(k.to_string(), v.to_string());
        }
        Self(data)
    }

    pub fn one<K: ToString, V: ToString>(k: K, v: V) -> Self {
        let mut data = HashMap::with_capacity(1);
        data.insert(k.to_string(), v.to_string());
        Self(data)
    }

    pub fn add<K: ToString, V: ToString>(&self, k: K, v: V) -> Self {
        let mut data = self.0.clone();
        data.insert(k.to_string(), v.to_string());
        Self(data)
    }

    pub fn merge(&self, x2: EnvData) -> Self {
        Self(self.0.clone().into_iter().chain(x2.0).collect())
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.0.get(k)
    }

    pub fn parent() -> Self {
        let env = std::env::vars();
        let mut data = HashMap::new();
        for (k, v) in env {
            data.insert(k, v);
        }
        Self(data)
    }
}

impl IntoIterator for EnvData {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub mod path {
    use crate::EnvData;

    fn del() -> &'static str {
        if cfg!(unix) {
            ":"
        } else if cfg!(windows) {
            ";"
        } else {
            panic!("Unsupported operating system")
        }
    }

    pub fn get() -> Option<String> {
        EnvData::parent().get("PATH").map(|x| x.to_owned())
    }

    pub fn extend(x: impl ToString) -> String {
        match self::get() {
            Some(path) => format!("{}{}{}", path, del(), x.to_string()),
            None => x.to_string(),
        }
    }
}
