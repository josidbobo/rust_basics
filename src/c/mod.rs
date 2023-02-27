pub mod closure{
    pub fn close(){
        // basic function to return true or false based on the value passed
        let can_vote = |age : i16| -> bool{
            age >= 18
        };
        println!("Can vote : {}", can_vote(4));

    let mut samp1 = 5;
    let print_var = || println!("sample1 = {}", samp1);
    print_var();

    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("Samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);

    // function indicating passing a function as a parameter using generic
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32{
        func(a, b)
    }

    let sum = |a , b| a+b;
    let prod = |a, b| a * b;
    println!("5 + 4 = {:#?}", use_func(5,4, sum));
    println!("5 x 7 = {:#?}", use_func(5,7, prod));

    }
}