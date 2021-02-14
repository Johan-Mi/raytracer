use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Var<T> {
    Value(T),
    Ref(String),
}
