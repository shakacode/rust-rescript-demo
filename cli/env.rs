use std::collections::HashMap;

pub type Env = HashMap<String, String>;

pub fn empty() -> Env {
    HashMap::new()
}

pub fn new<K: ToString, V: ToString>(kvs: Vec<(K, V)>) -> Env {
    let mut vars = HashMap::with_capacity(kvs.len());
    for (k, v) in kvs {
        vars.insert(k.to_string(), v.to_string());
    }
    vars
}

pub fn one<K: ToString, V: ToString>(k: K, v: V) -> Env {
    let mut vars = HashMap::with_capacity(1);
    vars.insert(k.to_string(), v.to_string());
    vars
}

pub fn merge(x1: Env, x2: Env) -> Env {
    x1.into_iter().chain(x2).collect()
}

pub fn parent() -> Env {
    let envs = std::env::vars();
    let mut vars = HashMap::new();
    for (k, v) in envs {
        vars.insert(k, v);
    }
    vars
}

pub mod path {
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
        super::parent().get("PATH").map(|x| x.to_owned())
    }

    pub fn extend(x: impl ToString) -> String {
        match self::get() {
            Some(path) => format!("{}{}{}", path, del(), x.to_string()),
            None => x.to_string(),
        }
    }
}
