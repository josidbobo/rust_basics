#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() { 
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("Didn't receive input");
    println!("Hello, {}! {}", name.trim_end(), greeting);

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.1493;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");

    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL); 

    let random_num : i32 = rand::thread_rng().gen_range(1..34);
    println!("The random num is {}", random_num);

    let age2 : i32 = 32;
    match age2 {
        1..=18 => println!("Underage child"),
        34..=i32::MAX => println!("Golden Jubilee"),
        _ => println!("This is the unhandled case"),
    }

    let voting_age = 18;
    match age2.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can Vote"),
        _ => println!("Not involved"),
    }; 

    let arr_1 : [i32; 9] = [1,2,3,4,5,6,7,8,9];
    println!("Length : {}", arr_1.len());

    let mut loop_idx = 0;
    loop {
        if arr_1[loop_idx] % 2 == 0{
            loop_idx +=1;
            continue;
        }
        if arr_1[loop_idx] == 9{
            break;
        }
        println!("Val : {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    // while statement
    let mut loop_idx: usize = 0;
    while loop_idx < arr_1.len(){
        println!("Arr element at {} is {}", loop_idx, arr_1[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_1.iter(){
        println!("Val : {} ", val);
    }

    /// tuples
    let my_tuple: (u8, String, f64) = (47, "Tobi".to_string(), 50_000.09);
    println!("Tuple element {}", my_tuple.1);
    let(r1, r4, r5) = my_tuple;
    println!("New element attached {}", r4);

    // Strings
    let mut st1 = String::new();
    st1.push('R');
    st1.push_str("equirement");
    for word in st1.split_whitespace(){
        println!("WORD IS {}", word);  
    }
    let st2 = st1.replace("R", "B");
    println!("New string {}", st2);


    let st3 = String::from("s t y g h e k k q i");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1{
        println!("{}", char);
    }

    let st4 = "Random String";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    // Empty the string
    st5.clear();
    let st6 = String::from("Just Some");
    let st7 = String::from("words");
    let st8 = st6 + &st7;

    for char in st8.bytes(){
        println!("{}", char);
    }

    let int_u8: u8 = 5;
    let int2_u8: u8 = 3;
    let int_u32 = (int_u8 as i32) + (int2_u8 as i32);

    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday, 
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self{
                Day::Sunday | Day::Saturday => true,
                _ => false, 
            }
        }
    }

    let today: Day = Day::Monday;
    match today{
         Day::Monday => println!("Everyone hates Monday"),
         _ => println!("Day is not Monday"),
    }
    println!("{}", today.is_weekend());
    
      





}
