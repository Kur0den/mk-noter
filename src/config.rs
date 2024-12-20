use serde::Deserialize;
use std::{collections::HashMap, hash::Hash};

pub trait ToHashMap {
    fn to_hashmap(&self) -> HashMap<String, String>;
}

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub settings: Settings,
    pub instances: Vec<Instance>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Settings {
    pub cw: CwSettings,
    pub default_visibility: Visibility,
    pub default_instance: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct CwSettings {
    pub always_enable: bool,
    pub default_content: String,
}

#[derive(Deserialize, Debug)]
pub enum Visibility {
    Public,
    Home,
    Followers,
    Specified,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct Instance {
    pub name: String,
    pub address: String,
    pub token: String,
}

impl ToHashMap for Instance {
    fn to_hashmap(&self) -> HashMap<String, String>{
        let mut map = std::collections::HashMap::new();
        map.insert("name".to_string(), self.name.clone());
        map.insert("address".to_string(), self.address.clone());
        map.insert("token".to_string(), self.token.clone());
        return map;
    }
}
