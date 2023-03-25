use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
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
            Self::Value(t) => t,
            Self::Ref(var_name) => env.get(&var_name).unwrap().clone(),
        }
    }
}

impl<T> Var<T> {
    pub fn map<U, F>(self, f: F) -> Var<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Value(t) => Var::Value(f(t)),
            Self::Ref(var_name) => Var::Ref(var_name),
        }
    }
}
