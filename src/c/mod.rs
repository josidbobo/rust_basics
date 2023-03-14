

pub mod closure{
    use std::{thread, time::Duration};

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

    pub fn binary() {
        pub struct TreeNode<T> {
            pub left: Option<Box<TreeNode<T>>>,
            pub right: Option<Box<TreeNode<T>>>,
            pub key: T,
        }

        impl<T> TreeNode<T>{
            pub fn new(key: T) -> Self{
                TreeNode { left: None, right: None, key }
            }
            pub fn left(mut self, node: TreeNode<T>) -> Self{
                    self.left = Some(Box::new(node));
                    self
            }
            pub fn right(mut self, node: TreeNode<T>) -> Self{
                    self.right = Some(Box::new(node));
                    self
                }
            }

            let node1 = TreeNode::new(1).
            left(TreeNode::new(0)).right(TreeNode::new(2));

            pub struct Bank{
                balance: f32,
            }

            fn withdraw(the_bank: &mut Bank, amt: f32){
                the_bank.balance -= amt;
            }
            let mut bank: Bank = Bank{balance: 100.0};
            withdraw(&mut bank, 5.0);
            println!("Balance is {}", bank.balance);

            let customer = |amt : f32|{
                withdraw( &mut bank, amt);
                println!("Balance is {}", bank.balance)
            };
            

        }
                
    }
