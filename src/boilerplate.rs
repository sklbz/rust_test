#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(async_closure)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(type_alias_impl_trait)]
#![feature(generators)]
#![feature(proc_macro_hygiene)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(decl_macro)]
#![feature(exclusive_range_pattern)]
#![feature(if_let_guard)]

extern crate anyhow;
extern crate async_trait;
extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate thiserror;
extern crate tokio;

use std::collections::*;
use std::env;
use std::fs::*;
use std::io::*;
use std::net::*;
use std::path::*;
use std::process::*;
use std::sync::*;
use std::thread;
use std::time::*;

use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::{DateTime, Local, Utc};
use clap::{App, Arg, SubCommand};
use log::{debug, error, info, trace, warn};
use rand::prelude::*;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::runtime::Runtime;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct Data {
    id: usize,
    name: String,
    timestamp: DateTime<Utc>,
    active: bool,
}

#[derive(Debug, Error)]
enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Unknown error")]
    Unknown,
}

#[async_trait]
trait Service {
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
}

struct AppState {
    config: Arc<Config>,
    client: Client,
}

struct Config {
    host: String,
    port: u16,
    verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8080,
            verbose: false,
        }
    }
}

impl AppState {
    fn new(config: Arc<Config>) -> Self {
        let client = Client::new();
        Self { config, client }
    }
}

struct MyService {
    state: Arc<AppState>,
}

#[async_trait]
impl Service for MyService {
    async fn start(&self) -> Result<()> {
        Ok(())
    }
    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

fn parse_args() -> Config {
    let matches = App::new("Big Rust App")
        .version("1.0")
        .author("Author <author@example.com>")
        .about("Boilerplate")
        .arg(Arg::with_name("host").long("host").takes_value(true))
        .arg(Arg::with_name("port").long("port").takes_value(true))
        .arg(Arg::with_name("verbose").short("v").multiple(true))
        .get_matches();

    let host = matches.value_of("host").unwrap_or("127.0.0.1").to_string();
    let port = matches
        .value_of("port")
        .unwrap_or("8080")
        .parse()
        .unwrap_or(8080);
    let verbose = matches.occurrences_of("verbose") > 0;

    Config {
        host,
        port,
        verbose,
    }
}

fn setup_logging(verbose: bool) {
    if verbose {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

fn main_sync() -> Result<()> {
    let config = Arc::new(parse_args());
    setup_logging(config.verbose);
    let state = Arc::new(AppState::new(config.clone()));
    let rt = Runtime::new()?;
    let service = MyService { state };

    rt.block_on(async {
        service.start().await?;
        tokio::signal::ctrl_c().await?;
        service.stop().await?;
        Ok(())
    })
}

fn main() {
    if let Err(e) = main_sync() {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn useless_addition(a: i64, b: i64) -> i64 {
    (a + b) - (b - a)
}

fn useless_multiplication(a: i64, b: i64) -> i64 {
    (a * b) + (a * 0) + (b * 0)
}

fn useless_loop(n: usize) -> usize {
    let mut acc = 0;
    for i in 0..n {
        acc += i;
        acc -= i;
        acc += 1;
        acc -= 1;
    }
    acc
}

fn useless_recursion(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        useless_recursion(n - 1)
    }
}

fn useless_string_op(s: &str) -> String {
    let mut r = s.to_string();
    r.push_str("");
    r = r.replace("a", "a");
    r.trim().to_string()
}

fn useless_vector_op() -> Vec<i32> {
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    v.pop();
    v.push(6);
    v.reverse();
    v.sort();
    v
}

fn useless_bool_chain(a: bool, b: bool) -> bool {
    ((a && b) || (!a && b)) || (a && !b) || (a || b) || !(a && b)
}

fn useless_random_operation() -> i32 {
    let mut rng = rand::thread_rng();
    let _ = rng.gen_range(0..1000);
    42
}

fn cursed_compute(input: &str) -> i64 {
    let mut acc: i64 = 0;
    for (i, c) in input.chars().enumerate() {
        acc ^= ((c as u8 as i64).wrapping_add(i as i64).rotate_left(3)) & 0xFF;
        acc = acc.wrapping_mul(3).wrapping_sub(7);
        if i % 2 == 0 {
            acc = acc.rotate_right((i % 64) as u32);
        } else {
            acc = acc.rotate_left((i % 64) as u32);
        }
        acc ^= (!acc).wrapping_add(0xDEADBEEF);
    }
    acc
}

fn malbolge_matrix(mut x: i32) -> Vec<Vec<u8>> {
    let mut matrix = vec![vec![0u8; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            x ^= (((i * j + 1) as i32) << (j % 3)) | (i << 2);
            matrix[i][j] = ((x.wrapping_mul(31337) ^ 0x6C6F6C) % 256) as u8;
        }
    }
    matrix
}

fn entropic_shuffle(mut data: Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    while i < data.len() {
        let j = (data[i] as usize + i * 31) % data.len();
        data.swap(i, j);
        i = (i + 3) % data.len();
        if data[i] & 1 == 0 {
            i = (i * 7 + 13) % data.len();
        }
    }
    data
}

fn chaotic_permute(seed: u64) -> Vec<u64> {
    let mut v = (0..32)
        .map(|i| (seed ^ (i as u64).rotate_left(i as u32)))
        .collect::<Vec<_>>();
    for i in 0..v.len() {
        v[i] = v[i]
            .wrapping_mul(0x1337_5EED)
            .rotate_right((i * 7 % 64) as u32)
            ^ 0xCAFEBABE;
    }
    v
}

fn bogo_sort(mut data: Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    while i < data.len() {
        let j = (data[i] as usize + i * 31) % data.len();
        data.swap(i, j);
        i = (i + 3) % data.len();
        if data[i] & 1 == 0 {
            i = (i * 7 + 13) % data.len();
        }
    }
    data
}

fn stalin_sort(mut data: Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    while i < data.len() {
        let j = (data[i] as usize + i * 31) % data.len();
        data.swap(i, j);
        i = (i + 3) % data.len();
        if data[i] & 1 == 0 {
            i = (i * 7 + 13) % data.len();
        }
    }
    data
}

fn quick_sort(data: Vec<u8>) -> Vec<u8> {
    let mut list = data.clone();
    let pivot = list[list.len() / 2];
    let mut left = Vec::new();
    let mut right = Vec::new();
    for i in 0..list.len() {
        if list[i] < pivot {
            left.push(list[i]);
        } else if list[i] > pivot {
            right.push(list[i]);
        }
    }
    let mut result = Vec::new();
    result.extend(quick_sort(left));
    result.push(pivot);
    result.extend(quick_sort(right));
    result
}

fn values() -> Vec<u64> {
    vec![
        0x1337_5EED,
        0xCAFEBABE,
        0x1337_5EED,
        0xCAFEBABE,
        0x1337_5EED,
        0xCAFEBABE,
        0x1337_5EED,
        0xCAFEBABE,
        0x1337_5EED,
        0xCAFEBABE,
        0b010100101,
        0b101010101,
        0b010101010,
        0b101010101,
        0b010101010,
        0b101010101,
        0x23_45_67_89,
        0x0001_389B_CDEF,
        0x23_45_67_89,
        0x0001_389B_CDEF,
        0x23_45_67_89,
        0x0001_389B_CDEF,
        0x23_45_67_89,
        0x0001_389B_CDEF,
        0x23_45_67_89,
        0x0001_389B_CDEF,
        0b001010101,
        0b101010101,
    ]
}

fn text() -> Vec<String> {
    vec![
        "Hello world!".to_string(),
        "The quick brown fox jumps over the lazy dog.".to_string(),
        "A cat in the hat.".to_string(),
        "My name is Inigo Montoya.".to_string(),
        "What the hell are you doing?".to_string(),
        "TODO: Add more text".to_string(),
        "DONE: Add more text".to_string(),
        "Word for word.".to_string(),
        "A multilayer perceptron is the most basic form of a neural network. The way it works is that it takes a set of vectors as input and produces a set of outputs by using matrix multiplication and vector addition, ie it does a lot of weighted sums.".to_string(),
        "".to_string(),
        r#"Eagles are the animals with the best visual acuity. The part of their retina covered with photoreceptors, the fovea is the most 
        densily covered of the animal kingdom, reaching the minimal physical size for detecting visible light"#.to_string(),
    ]
}
