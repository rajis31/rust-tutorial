#![allow(unused)]  // turns off warnings for ununsed variables

use std::io; 

fn main() {
    
    // VARIABLES
    // Variables are mutable by default

    let x = 6;
    println!("{:?}", x);

    let mut y: i32 = 6;  // Making variable mutable
    println!("{:?}", y);
    y = 10; 
    println!("{:?}", y);

    let lunch:String  = String::from("LUNCH IS READY");
    println!("{}", lunch);

    const TOTAL_CARS: i32 = 6; 
    println!("{}", TOTAL_CARS);

    // Shadowing 
    // It basically creates new variables
    // Does not work like mutable variables
    let x:i32 = 10; 
    let x:i32 = x + 1; 
    println!("{}", x);



    // FUNCTIONS

    // Functions signature must specify types 
    // -> specify the return type
    fn sum(x: i32, y: i32)-> i32{
        return x + y;
    }

    fn sum1(x: f32, y: f32)->f32{
        return x+y;
    }

    println!("{:?}", sum(1,3));
    println!("{:?}", sum1(1.0,3.0));


    // CONTROL FLOW
    let z = 5; 

    // if z { // rust wont convert Non-Bool type to bool types
    //     println!("{z} is above 0"); 
    // }

    if z>=5 {
        println!("{z} is greater then or equal to 5");
    }





    
}
