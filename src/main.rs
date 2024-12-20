pub mod config;
pub mod input;
pub mod print;

use std::{collections::HashMap, env, io::{self, Read, Write}, ptr::read};
use std::fs::File;
use config::{Config, Instance, ToHashMap};
use colored::*;

fn main() {

    let config: Config = check_config();
    // let input = input::read_line("Please enter some text here");

    // println!("You entered: {}", input.trim());
    // println!("Entered string length: {}", input.trim().len());

    println!("{:?}", config);

    config_create();

}


fn check_config() -> Config{
    let contents = read_file().unwrap();
    toml::from_str(&contents).unwrap()
    // return config;

    // match read_file() {
    //     Ok(contents) => {
    //         let config: Config = toml::from_str(&contents).unwrap()
    //     },
    //     Err(e) => {
    //         toml::from_str(&contents).unwrap()
    //         // Err(Box::new(e));
    //     }
    // }
}

fn read_file() -> Result<String, Box<dyn std::error::Error>>{
    const FILEPATH: &str = "config.toml";
    // let args: Vec<String> = env::args().collect();
    let mut f = File::open(FILEPATH)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    // ファイルの読み込み中に問題がありました
    // .expect("something went wrong reading the file")

    Ok(contents)
}


fn config_create() {
    let mut config: Config;
    print::warning("Configファイルが存在しないか、読み込めませんでした");
    println!("新規にConfigファイルを作成します");

    let mut instance: Instance = Instance::default();
    print::title("インスタンス設定");
    println!("デフォルトで使用するプロファイルを設定します");
    loop {

        print::hint("インスタンスのアドレスは、chpk.kur0den.net のようなものです");
        instance.address = input::read_line("インスタンスのアドレスを入力してください");
        instance.token = input::read_line("ノート作成権限を付与したトークンを入力してください");
        print::hint("投稿先選択時に表示される名前になります");
        instance.name = input::read_line("このプロファイルに設定する名前を入力してください");


        println!("{:?}", instance);

        print::list(instance.to_hashmap());
        if input::confirm("この内容でプロファイルを保存しますか?") {
            break
        }else {
            print::info("再登録を行います");
        }
    }



}
