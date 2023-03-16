// ALT + SHIF + UP will copy and move a given line or highlighed lines
// ALT + ARROW will move the copied line/lines in any direction
pub mod closure {
    use std::{thread, time::Duration, cmp::Ordering};
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};

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

    /// Binary Tree implementation in Rust
    pub fn binary(){

        // A Smart pointer (BOX) gives added functionalities to a referenced item not just storing the data on the heap
        // but also storing the POINTER on the stack
        pub struct TreeNode<T> {
            pub left: Option<Box<TreeNode<T>>>,
            pub right: Option<Box<TreeNode<T>>>,
            pub key: T,
        }

        impl<T> TreeNode<T>{
            pub fn new(key: T) -> Self{
                TreeNode { left: None, right: None, key }
            }

            pub fn left(mut self, node: TreeNode<T>) -> Self {
                    self.left = Some(Box::new(node));
                    self
            }

            pub fn right(mut self, node: TreeNode<T>) -> Self {
                    self.right = Some(Box::new(node));
                    self
                }
            }

            let node1 = TreeNode::new(1).
            left(TreeNode::new(0)).right(TreeNode::new(2));

    println!("Thread and Concurrency starts here");
            let thread1 = thread::spawn(|| {
                for i in 1..25{ 
                    println!("Spawned thread: {}", i);
                    thread::sleep(Duration::from_millis(1))
                }
            });
            for i in 1..20{
                println!("Main thread {}", i);
                thread::sleep(Duration::from_millis(1));
            } 
        
            thread1.join().unwrap();

            pub struct Bank{
                balance: f32,
            }

            fn withdraw(the_bank: &mut Bank, amt: f32){
                //let y = true;
                match the_bank.balance > 0.00{
                     true => the_bank.balance -= amt,
                    _ => println!("The amount is above balance"),
                }
            }
            let mut bank: Bank = Bank{balance: 213.00};
            withdraw(&mut bank, 5.0);
            println!("Balance is {}", bank.balance);

        pub fn customer(amt: f32, bbank: &mut Bank){
                withdraw( bbank, amt);
                println!("Balance is {}", bbank.balance);
            }

        customer(45.8, &mut bank);
            
        }

        /// Smart Pointers implementation Rust 
        pub fn ref_call(){
            println!("This is where Smart pointers function starts");
            struct Bank{
                balance: f32
            }
                
           // Arc - provides shared ownership of same value.
            fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
                let mut bank_ref = the_bank.lock().unwrap();
                match bank_ref.balance < 5.00{
                    true => println!("Currrent Balance : {} withdraw smaller amount", bank_ref.balance),
                    _ => {
                        bank_ref.balance -= amt;
                        println!("Customer withdrew : {}, Current Balance : {}", amt, bank_ref.balance);
                    }
                }
            }

                fn customer(the_bank: Arc<Mutex<Bank>>){
                    withdraw(&the_bank, 14.0);
                }
                let bank = Arc::new(Mutex::new(Bank {balance: 200.00}));
                let handles = 
                (0..10).map(|_|{
                    let bank_ref = bank.clone();
                    thread::spawn(||{
                        customer(bank_ref)
                    })
                });

                for handle in handles{
                    handle.join().unwrap()
                }

                println!("Total {}", bank.lock().unwrap().balance);


        }
                
    }
