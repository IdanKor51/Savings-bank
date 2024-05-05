//use std::thread::sleep;
//use std::time::Duration;
mod pretty_text;
mod tools;
// get parameters
//use std::env;
//let args: Vec<String> = env::args().collect();

#[allow(unused)]
fn main(){
    let mut todo_list: Vec<String> = Vec::new();
    'add_to_list: 
        loop {
            let out: String = tools::read_inp().unwrap();
            todo_list.push(out);
            if todo_list.len() >10 {break};
            pretty_text::precentage((todo_list.len()*10)as f32);
        }
    println!("too many elements");
}
