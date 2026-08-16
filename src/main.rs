extern crate core;

use crate::builders::polling_rate_builder::{PollingRateBuilder, Rate};
use crate::users::User;
use core::fmt;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{Error, Read};
use std::os::unix::fs::FileExt;
use uuid::Uuid;

mod builders;
mod users;

const CONFIG_FILE_PATH: &str = "./config.txt";

fn main() {
    let mut builder = PollingRateBuilder::new(Rate::Gaming);
    builder.set_rate(Rate::K8);

    println!("Rate: {:?}", builder.rate);

    let mut user = User::new(
        Uuid::new_v4(),
        "harukadev511",
        "harukadev",
        true,
        1785756736,
    );

    user.change_nickname("ola").toggle_is_active();

    println!("{}", user);

    let mut users: Vec<User> = Vec::new();
    users.push(user);

    println!("{:?}", users);

    let ref_user: &User = &users[0];
    let ref_user: Option<&User> = users.get(0);

    match ref_user {
        Some(ref_user) => println!("User localized!"),
        None => println!("User no found!"),
    }

    let v: Vec<u8> = vec![0xff, 0x32, 0x57];
    for i in &v {
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    static ICON: &[u8] = include_bytes!("main.rs");
    let data = "Initial content";
    let mut s = data.to_string();

    println!("Length: {}", String::from("السلام عليكم").len());
    println!("Length: {}", String::from("Dobrý den").len());
    println!("Length: {}", String::from("Hello").len());
    println!("Length: {}", String::from("שלום").len());
    println!("Length: {}", String::from("नमस्ते").len());
    println!("Length: {}", String::from("こんにちは").len());
    println!("Length: {}", String::from("안녕하세요").len());
    println!("Length: {}", String::from("你好").len());
    println!("Length: {}", String::from("Olá").len());
    println!("Length: {}", String::from("Здравствуйте").len());
    println!("Length: {}", String::from("Hola").len());

    s.push_str("a");
    print!("{}\n", data);

    s.push('a');

    let d1 = String::from("Hello, ");
    let d2 = String::from("world!");

    let d3 = d1 + &d2;
    println!("{}", d3);

    let hello = String::from("Здравствуйте");
    let answer = &hello[0..2];
    println!("{}", answer);

    for c in hello.chars() {
        println!("{c}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("red"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("red");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score {score}");

    // let discord_pipe = File::open("/run/user/1000/discord-ipc-0");

    match File::open("Cargo.toml") {
        Ok(mut file) => {
            let mut string = String::new();
            let n = file.read_to_string(&mut string);
            println!("{:?}", string);
            println!("{:?}", n.unwrap_or(0))
        }
        Err(err) => println!("File not opened"),
    }

    set_value("API_KEY", "65465465464646").expect("TODO: panic message");

    let value = get_value("API_KEY");
    match value {
        Some(text_value) => println!("Get Value: {}", text_value),
        None => {
            println!("{:?}", value)
        }
    }
    set_value("REDIS_PASSWORD", "65465465464646").expect("TODO: panic message");
    set_value("API_KEY", "aaaaaa").expect("TODO: panic message");

    let largest_numbers = vec![0, 10, 2, 3, 4, 5, 6];
    let largest = largest_i32(&largest_numbers);
    println!("largest number: {}", largest);

    let largest_numbers_f32: Vec<f32> = vec![0.0, 10.0, 2.2, 3.9, 4.4, 5.9, 6.4];
    let largest = get_largest(&largest_numbers_f32);

    let largest_numbers_char: Vec<char> = vec!['a', 'b', 'c', 'z'];
    let largest = get_largest(&largest_numbers_char);
    println!("largest char generic: {}", largest);

    let pair = Pair {
        first: 'a',
        second: 'b',
    };

    println!("pair: {}", pair)
}
fn get_value(key: impl Into<String>) -> Option<String> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(CONFIG_FILE_PATH)
        .expect("Problem opening the file: ");

    let mut config_file_raw = String::new();

    file.read_to_string(&mut config_file_raw)
        .expect("Problem reading the file: ");

    if config_file_raw.rfind("=").is_none() || config_file_raw.len() < 3 {
        return None;
    }

    let lines: Vec<&str> = config_file_raw.split("\n").collect();

    let key_pattern = format!("{}=", key.into());

    let key_line = lines.into_iter().find(|x| x.starts_with(&key_pattern));

    Some(key_line?.to_string().replace(&key_pattern, ""))
}

fn set_value(key: impl Into<String>, value: impl Into<String>) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(CONFIG_FILE_PATH)
        .expect("Problem opening the file: ");

    let mut config_file_raw = String::new();
    file.read_to_string(&mut config_file_raw)
        .expect("Problem reading the file: ");

    let key = key.into();
    let value = value.into();

    let new_line = format!("{}={}", &key, &value);
    let len = file.metadata()?.len();

    let value_result = get_value(&key);

    match value_result {
        Some(value) => {
            let new_config_file_raw = config_file_raw.replace(
                &format!("{}={}", &key, value).to_string(),
                new_line.as_ref(),
            );

            file.write_all_at(new_config_file_raw.as_bytes(), 0)?;
        }
        None => {
            let text = format!("\n{}={}", &key, &value);
            file.write_all_at(text.as_bytes(), len)?;
        }
    }

    Ok(())
}
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if (item > largest) {
            largest = item;
        }
    }
    largest
}

fn get_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Display> Display for Pair<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({} with {})", self.first, self.second)
    }
}
