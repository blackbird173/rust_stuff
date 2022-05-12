use std::{io, cell::RefCell};
fn main() {
    #[derive(PartialEq)]
    // make a struct that has an enum called options and the enum has two choices and the struct also has something called prizes and its innmutable
    // option 1 gives u a number that's max is 10000 and the min is -10000 (u will write ur own function and u will not use clamp and u must use trains)
    // option 2 gives u a number that is odd and if it is even it either goes down or up however it will always be even
    // user input select 1 or 2
    // no using mut only refcell
    enum Options {
        Option1,
        Option2,
    }
    struct Prizes {
        choice: Options,
        prize: String,
    }
    let option = RefCell::new(Prizes{choice: Options::Option1, prize: String::from("u get a random number which max's is 10000")});
    let inp = RefCell::new(String::new());
    io::stdin().read_line(&mut inp.borrow_mut()).unwrap();
    trait X {
        fn clam(num: RefCell<Self>, min: i32, max: i32) -> i32;
    }
    impl X for i32 {
        fn clam(num: RefCell<Self>, min: i32, max: i32) -> i32 {
            if *num.borrow_mut() < min {
                *num.borrow_mut() = min;
                return *num.borrow_mut();
            } else if *num.borrow_mut() > max {
                *num.borrow_mut() = max;
                return *num.borrow_mut();
            } else {
                return *num.borrow_mut();
            }
        }
    }
    let prize_num = 0;
    let x = &prize_num as *const i32;
    let f = x as i32 / 10;
    if inp.borrow_mut().trim().parse() == Ok(2) {
        option.borrow_mut().choice = Options::Option2;
        option.borrow_mut().prize = String::from("if the rand number is odd it goes down but if its even it goes up");
    }
    if option.borrow_mut().choice == Options::Option1 {
        let f = i32::clam(RefCell::new(f), -10000, 10000);
        println!("the prized number is {}", f);
        println!("the prize is {}", option.borrow_mut().prize);
    } else if option.borrow_mut().choice == Options::Option2 {
        if f%2 == 0 {
            println!("f's old value {}", f);
            println!("f's new value {}", f+1);
            println!("even");
        } else {
            println!("f's old value {}", f);
            println!("f's new value {}", f-1);
            println!("odd");
        }
        println!("the prize is {}", option.borrow_mut().prize);
    }
}
