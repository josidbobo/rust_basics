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
    heroes.insert("Batman", "Amigos");
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

pub fn struct_s(){
    const PI: f32 = 3.141592;
    struct SizedRectangle<T, U>{
        height: T,
        width: U,
    }

    struct Rectangle{ length: f32, width: f32}
    struct Circle{length: f32, width: f32}

    let rec = SizedRectangle{
        height: 4, width: 45.3
    };

    trait Shape{
        // The new function serves as a instantiating constructor
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    impl Shape for Rectangle{
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32{
            return (self.length * self.width);
        }
    }

    impl Shape for Circle{
        fn new(length: f32, width: f32) -> Circle{
            return Circle {length,width};
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI * 2.0;
        }
    }


    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer{
        name: String::from("Maazi"),
        address: String::from("4, Ekwueme street"),
        balance: 45.4,
    };

    bob.address = String::from("5, Chukwujike street");



}