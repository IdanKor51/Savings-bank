use std::env;
//use std::thread::sleep;
//use std::time::Duration;
mod pretty_text;
fn main(){
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<i32>().unwrap();
    pretty_text::precentage(n as f32);
}