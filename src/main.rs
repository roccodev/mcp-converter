/* Extern crates */
extern crate regex;

/* Link libraries */
mod lib;

fn main() {
    println!("Running.");
    println!("{}", lib::srg::find_class_notchian("lo").unwrap().mcp_name);
}