use tokio::time;
// use tokio::runtime::Runtime;
// use chrono::prelude::*;
// use sqlx::mysql::MySqlPoolOptions;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
// use std::vec;

#[tokio::main]
async fn main() {
    // let rt = Runtime::new().unwrap();
    // rt.block_on(result());
    result().await;

    // rt.block_on(my_function());
    my_function().await;

    let path = "src/file.txt";
    read_and_write_file(path).unwrap();

    let word = "kak";
    if is_palidrome(word) {
        println!(" ");
    }
}

async fn run() -> i32 {
    time::sleep(time::Duration::from_secs(3)).await;
    32
}

fn cal(num: i32) -> i32 {
    num * num
}

async fn result() {
    let val = run().await;
    let result = cal(val);
    println!("{}", result);
}

async fn my_function() {
    let s1 = connect_to_server().await;
    println!("first result: {s1}");
    let s2 = connect_to_server().await;
    println!("second result: {s2}");
}

async fn connect_to_server() -> String {
    String::from("DB Ressult")
}

fn read_and_write_file(path: &str) -> std::io::Result<()> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut result = String::new();
    reader.read_to_string(&mut result)?;

    let changed = result.to_uppercase();

    delete(path)?;

    let new_file = File::create(path)?;

    let mut write = BufWriter::new(new_file);

    write.write_all(changed.as_bytes())?;

    Ok(())
}

fn delete(path: &str) -> Result<(), std::io::Error> {
    std::fs::remove_file(path)
}

fn is_palidrome(str: &str) -> bool {
    let s = str.chars().collect::<Vec<char>>();
    // println!("{:?}", &s);
    s == s.iter().rev().cloned().collect::<Vec<char>>()
}
