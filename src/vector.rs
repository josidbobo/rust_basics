#![allow(unused)]

use::std::fmt::Display;

pub fn v_ector_and_others() {

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];

    let second = &vec2[1];
    match vec2.get(1){
        Some(second) =>  println!("There is a second {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2{
        *i *= 2;
        println!("i is {}", i)
    }
    for i in &vec2{
        println!("{}", i);
    }

    println!("Vec length {}", vec2.len());
    // {:?} Gets the item to be removed or referenced
    println!("Pop operation element {:?}", vec2.pop());
}

