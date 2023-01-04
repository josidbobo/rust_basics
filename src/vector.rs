#![allow(unused)]

pub fn v_ector_and_others() -> String {

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 : Vec<i32> = vec![1,2,3,4];
    vec2.push(5);
    println!("1st {}", vec2[0]);

    let second = &vec2[1];
    match vec2.get(1){
        Some(second) =>  second.to_string(),
        None => String::from("No 2nd value"),
        _ => String::from("Last option"),
    }






}