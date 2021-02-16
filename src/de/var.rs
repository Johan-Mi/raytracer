use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Var<T> {
    Value(T),
    Ref(String),
}

impl<T> Var<T>
where
    T: Clone,
{
    pub fn resolve(self, env: &HashMap<String, T>) -> T {
        match self {
            Var::Value(t) => t,
            Var::Ref(var_name) => env.get(&var_name).unwrap().clone(),
        }
    }
}

impl<T> Var<T> {
    pub fn map<U, F>(self, f: F) -> Var<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Var::Value(t) => Var::Value(f(t)),
            Var::Ref(var_name) => Var::Ref(var_name),
        }
    }
}
