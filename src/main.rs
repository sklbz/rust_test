use std::fs::read;

fn main() {
    println!("Hello, world!");
    println!("viktor");

    let test = read("~/.config/nvim/init.lua").unwrap();
    println!("{:?}", test);
}
