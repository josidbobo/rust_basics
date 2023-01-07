#![allow(unused)]

use::std::ops::Add;
use::std::collections::HashMap;

pub fn v_ector_and_others() {

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];

    println!("---This is the beginning of V_ector_and_others file---");

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
    println!("Vec elements are {:?}", vec2);
    // {:?} Gets the item to be removed or referenced
    println!("Pop operation element {:?}", vec2.pop());
}

pub fn get_sum_gen <T:Add<Output = T>>(x: T, y: T) -> T{
    return x + y;
}

pub fn hash_map(){
    let mut heroes = HashMap::new();
    heroes.insert("SuperSTAR", "Amigos");
    heroes.insert("Flash", "DCUniverse");

    for (k, v) in heroes.iter(){
        println!("heroes[{}] => {} ", k, v);
    }

    if heroes.contains_key("Batman"){
        let bat_man = heroes.get("Batman");
        match bat_man {
            Some(x) => println!("This is the value for Batman {}", x),
            None => println!("There is no mapping with that key"),
        }
    } else{
        println!("There is no batman and the match wasn't called");
    }

    


println!("----This is the end of v_ectors_and_others file----")
}