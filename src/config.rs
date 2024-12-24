use serde::Deserialize;
use indexmap::IndexMap;


// TODO: indexMap型で書き直し
pub trait ToReadableIndexMap {
    fn to_readable_indexmap(&self) -> IndexMap<String, String>;
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

impl ToReadableIndexMap for CwSettings {
    fn to_readable_indexmap(&self) -> IndexMap<String, String>{
        let mut map = IndexMap::new();
        map.insert("デフォルトで有効".to_string(), self.always_enable.clone().to_string());
        map.insert("デフォルトの注釈".to_string(), self.default_content.clone());
        return map;
    }
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

impl ToReadableIndexMap for Instance {
    fn to_readable_indexmap(&self) -> IndexMap<String, String>{
        let mut map = IndexMap::new();
        map.insert("プロファイル名".to_string(), self.name.clone());
        map.insert("アドレス".to_string(), self.address.clone());
        map.insert("トークン".to_string(), self.token.clone());
        return map;
    }
}
