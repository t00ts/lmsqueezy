use std::collections::HashMap;

pub const API_URL: &str = "https://api.lemonsqueezy.com";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JsonAPI {
    version: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Links {
    first: Option<String>,
    last: Option<String>,

    next: Option<String>,
    prev: Option<String>,

    #[serde(rename = "self")]
    _self: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Response<T> {
    pub jsonapi: Option<JsonAPI>,
    pub links: Option<Links>,
    pub data: Option<ResponseData<T>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct VecResponse<T> {
    jsonapi: Option<JsonAPI>,
    links: Option<Links>,
    data: T,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ResponseData<T> {
    pub r#type: String,
    pub id: String,
    pub relationships: Option<HashMap<String, HashMap<String, Relationship>>>,
    pub attributes: T,
    pub links: Option<Links>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Relationship {
    related: Option<String>,
    #[serde(rename = "self")]
    _self: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ResponseMeta<T> {
    jsonapi: Option<JsonAPI>,
    meta: T,
}
