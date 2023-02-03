mod closure{
    pub fn close(){
        let can_vote = |age : i16|{
            age >= 18
        };
        println!("Can vote : {}", can_vote(4));

    let mut samp1 = 5;
    let print_var = || println!("sample1 = {}", samp1);
    print_var();

    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var()

    }
}