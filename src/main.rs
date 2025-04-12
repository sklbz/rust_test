use std::fs::read;
mod boilerplate;

fn main() {
    println!("Hello, world!");
    println!("viktor");

    let test = read("~/.config/nvim/init.lua").unwrap();
    println!("{:?}", test);
}
