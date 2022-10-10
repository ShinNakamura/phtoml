use std::env;
use std::fs;
use std::collections::HashMap;

/// 第一引数は TOML
/// 第二引数は プレースホルダー`{%ph%}`がしこまれてるテンプレートファイル
/// プレースホルダーを TOML から読んだ値で埋めて、標準出力へ
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("need TOML and template");
        return;
    }
    let conf = &args[1];
    let conf = fs::read_to_string(conf).unwrap();
    let cm: HashMap<String,String> = toml::from_str(&conf).unwrap();
    let tmpl = &args[2];
    let tmpl = fs::read_to_string(tmpl).unwrap();

    let mut res = tmpl;
    for (k, v) in cm.iter() {
        res = res.replace(&format!("{{%{}%}}", k), v);
    }
    println!("{}", res);
} 
