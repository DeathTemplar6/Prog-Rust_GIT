use std::env;

fn main() {
    let first = env::args().nth(1).expect("please supply an argument");
    let n: i32 = first.parse().expect("not an integer!");
	println!("n {}", n);
	
	let second = env::args().nth(2).expect("please supply an argument");
    let n2: i32 = second.parse().expect("not an integer!"); 
	println!("n2 {}", n2);
}
/*
//test slice
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);
	println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());
	
	let maybe_last = slice.get(5);
    let _last = if maybe_last.is_some() {
        println!("maybe_last value {}",*maybe_last.unwrap());
    } else {
        println!("maybe_last value {}", "unset");
    };
	let s_last = *slice.get(5).unwrap_or(&-1);
	println!("s_last value {}", s_last);
}
*/
/* 
//base renvoit texte
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
*/