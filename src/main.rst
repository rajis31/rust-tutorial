#![allow(unused)]  // turns off warnings for ununsed variables

// Importing IO and Random library
// Crates are bundles of code

use std::io; 
use rand::Rng;
use std::io::{Write,BufReader,BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Import module and to use the order_food function
mod restaurant;
use crate::restaurant::order_food;
mod test;
use operations::subtraction;
use operations::optimization::optimization::custom as optCustom;

fn main() {

    subtraction::subtraction();
    test::war();
    optCustom::test();
}

   // printIn! is called a macro
    // macro's have a ! after the name

    // All variables declared inside of rust are immutable
    // but you can set them up to be mutable

    // All variable declarations start with "let"

    // Starting a variable name with underscore will tell the 
    // compiler to disregard it

    // .expect() is for error handling

    let mut name: String = String::new();
    let greeting: &str   = "Nice to meet you";
    const ONE_MIL: u32   = 1_000_000; // 32 bit unsigned integer constant
    const PI: f32 = 3.141592; // 32 bit float 
    let age: &str = "47"; // Strings are in double quotes. 
                         // Characters are single quote 
    let mut age: u32 = age.trim().parse()
                .expect("Age was not assigned a number");
    age = age + 1; 
    println!("I'm {} and I want ${}", age, ONE_MIL);
    println!("What is your name?");
    io::stdin().read_line(&mut name)
                .expect("Didn't Receive Input");
    println!("Hello, {}! {}", name.trim_end(), greeting);


    // Numbers
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u32::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    // Boolean
    let is_true: bool = true;

    // Character
    let my_grade: char = 'A';

    // Precision 
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);

    // Operations 
    let mut num_3: u32 = 5; 
    let num_4: u32 = 4; 

    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);
    num_3 += 1;

    // Random Numbers
    // Range is declared using .. 
    // It randonly generates a number between 1 to 100. 
    // Endpoint is not included
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);

    // Conditionals 
    let age: i32 = 8; 
    if(age >=1) && (age <= 18){
        println!("Important Birthday");
    } else if (age == 21) || (age == 50){
        println!("Important Birthday");
    } else{
        println!("Not important birthday");
    }

    // Ternary Operator 
    let mut my_age = 47; 
    let can_vote: bool = if my_age >= 18{
        true
    } else {
        false
    };
    println!("Can Vote: {}", can_vote);

    // Match 
    // Like a switch statement
    let age2: i32 = 21;
    match age2 {
        1..=18 => println!("Important Birthday 1 to 18"), // Between 1 and 18
        21 | 50 => println!("Important Birthday 21 or 50"), // 21 orr 50
        65..=i32::MAX =>println!("Important Birthday above 65"), // above 65
        _ => println!("Not an Important Birthday"), // no match
    };

    let my_age: i32 = 18;
    let voting_age: i32 = 18;

    // Here we are comparing voting_age to my_age 
    // Ordering::Less means if my_age < voting_age etc.
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater =>println!("Can vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };

    // Arrays
    let arr_1: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    // Loop array
    let mut loop_idx: usize = 0;

    loop{
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx+=1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }

        println!("Val: {}", arr_1[loop_idx]);
        loop_idx += 1;
    };


    loop_idx = 0;
    while loop_idx < arr_1.len(){
        println!("Arr: {}", arr_1[loop_idx]);
        loop_idx+=1;
    };

    for val in arr_1.iter(){
        println!("Val: {}", val);
    }

    // Tuples 
    // Contains multiple different datatypes of the same size 
    // .to_string() will convert &str to String.
    let my_typle: (u8 ,String, f64) = (47,"Derek".to_string(), 50_000.00);
    println!("Name: {}", my_typle.1); // Reference the name 

    // Deconstruct tuple
    let (v1,v2,v3) = my_typle;
    println!("Age: {}", v2);


    // Mutable Strings
    let mut st1: String  = String::new();
    st1.push('A');
    st1.push_str("word");
    for word in st1.split_whitespace(){
        println!("{}", word);
    }

    let st2: String = st1.replace("A", "Another");
    println!("{}", st2);

    let st3: String = String::from("x r t b h k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    for char in v1{
        println!("{}", char);
    }

    let st4: &str = "Random String";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_aray1: &[u8] = st5.as_bytes();
    let st6: &str = &st5[0..6]; // Slice string

    println!("String Length: {}", st6.len());
    st5.clear(); // Delete string 
    let st6: String  = String::from("Just some");
    let st7: String  = String::from(" words");
    let st8: String  = st6 + &st7; // Concatenate
    // In the above we reference &st7 since it exists 
    // st6 does not exist

    println!("{}", st8);


    // Casting 

let int_u8: u8 = 5;
let int2_u8: u8 = 4;
let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32); // Cast to u32 
println!("{}", int3_u32); 
    

    // Enums 

enum Day {
    Monday, 
    Tuesday, 
    Wednesday,
    Thursday,
    Friday, 
    Saturday, 
    Sunday
};


// Add methods to Enum
impl Day{
    fn is_weekend(&self)->bool{
        match self{
            Day::Saturday | Day::Sunday => true,
            _=> false
        }
    }
}

let today:Day = Day::Monday;
match today {
    Day::Monday => println!("Today is Monday"),
    Day::Tuesday => println!("Today is Tuesday"),
    Day::Wednesday => println!("Today is Wednesday"),
    Day::Thursday => println!("Today is Thursday"),
    Day::Friday => println!("Today is Friday"),
    Day::Saturday => println!("Today is Saturday"),
    Day::Sunday => println!("Today is Sunday")
};
println!("IS today the weekend {}", today.is_weekend());

// Vectors
// Grows like arrays but only hold same datatype 

let vec1: Vec<i32> = Vec::new();
let mut vec2: Vec<i32> = vec![1,2,3,4];
vec2.push(5);
println!("1st: {}", vec2[0]);
let second: i32 = vec2[1];
match vec2.get(1){
     Some(second) => println!("2nd: {}", second),
     None=> println!("No 2nd value")
}

for i in &mut vec2{
    *i *=2;
}

for i in &vec2{
    println!("{}", i);
}

println!("Vec length {}", vec2.len());
println!("Pop: {:?}", vec2.pop());

// Functions 
fn get_sum(x: i32, y: i32)->i32{
    println!("{} + {} = {}", x,y,x+y);
    return x+y; // Return statement
}

fn get_sum_2(x: i32) ->(i32, i32){
    return (x+1, x+2); // Return statement
}

fn get_sum_3(list: &[i32]) -> i32{
    let mut sum: i32 = 0; 

    for &val in list.iter(){
        sum += val;
    }

    return sum;

}

println!("{}", get_sum(2, 4));

let (val_1, val_2) = get_sum_2(3);
println!("Nums: {} {}", val_1, val_2);


let num_list = vec![1,2,3,4,5,5,6];
// To pass a vector element to a string use & to reference 
// the value. 

println!("Sum of List = {}", get_sum_3(&num_list));


// Generics 
    // Specifies the operation for different data types 
    // T is called a trait

    // Cover generics more aler
use std::ops::Add;
fn get_sum_4_generic<T: Add<Output = T>>(x:T,y:T) -> T{
    return  x+y;
}

println!("5 + 4 = {}", get_sum_4_generic(5, 4));

// Ownership 
    // Memory is going to managed by ownership 
    // with rules checked at compile time 

    // Stack - Stores values FIFO format 
    // Heap  - You request a certain amount of space and 
    //          it will then return an address for that space..

    // 1. Eacj value has a variable that;s called its owner 
    // 2. There is only one owner at a time
    // 3. When an onwer goes out of scope the value 
    //    disappears.

let str66: String = String::from("World");
let str67: String = str66;

// println!("{}",str66); // str66 does not exist 
// because str66 does not exist anymore.
// println!("{}",str67);

let str68: String = String::from("World");
let str69: String = str68.clone();


fn print_str(x: String){
    println!("A string {}", x);
}

fn print_return_str(x: String)->String{
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String){
    name.push_str(" is happy");
    println!("Message: {}", name);
}

print_str(str68);
let str70 = print_return_str(str69);
println!("{}",str70);

// hasmaps 
    // used to store key, value pairs 

use std::collections::HashMap;
let mut heros  = HashMap::new();
heros.insert("Superman", "Clark Kent");
heros.insert("Batman", "Bruce Wayne");
heros.insert("The Flash", "Barry Allen");

for(k,v) in heros.iter(){
    println!("{} = {}",k,v);
}

println!("Length: {}", heros.len());

if heros.contains_key(&"Batman"){
    let the_batman = heros.get(&"Batman");

    match the_batman{
        Some(x) => println!("Batman is a hero"),
        None => println!("Batman is not a hero")
    }
}


// Struct 
struct  Customer {
    name: String,
    address: String, 
    balance: f32,
}

let mut bob: Customer = Customer {
        name: String::from("Bob Smith"), 
        address: String::from("555 main street"), 
        balance: 234.50 
};
bob.address = String::from("505 main street");

 // Struct with generics 

struct  Rectangle<T,U>{
    length: T, 
    height: U
}

let rec = Rectangle{length: 4, height: 10.5};
trait Shape{
    fn new(length: f32, width: f32)-> Self; 
    fn area(&self) -> f32;
}

struct Rectangle1 {
    length: f32, 
    width: f32
}

struct Circle {
    length: f32, 
    width: f32
}

impl Shape for Rectangle1{
    fn new(length: f32, width: f32) -> Rectangle1{
        return Rectangle1{length, width};
    }

    fn area(&self) -> f32{
        return self.length * self.width;
    }
}

let rec: Rectangle1 = Shape::new(10.0, 9.0);
println!("Rec Area: {}", rec.area());


// Modules 
/***
 *  Crates: Modules that produce a library or a module 
 *  Modules: Organize and handle privacy
 *  Packages: Build, test, and share crates
 *  Paths: A way of naming an item such as a Function, Struct ,etc.
 */


  order_food();


  // Files 
  let path: &str = "lines.txt";
  let output = File::create(path);
  
  let mut output: File = match output{
    Ok(file) => file, 
    Err(error) => {
        panic!("Problem creating file :{}",error);
    }
  };
  write!(output, "Just Some\nRandom Words")
    .expect("Filed to write to file");

  let input = File::open(path).unwrap();
  let buffered = BufReader::new(input);

  for line in buffered.lines(){
    println!("{}",line.unwrap());
  }


  let output2 = File::create("rand.txt");

  let output2: File = match output2{
    Ok(file) => file, 
    Err(error) => match error.kind(){
        ErrorKind::NotFound => match File::create("rand.txt"){
            Ok(fc) =>fc,
            Err(e) => panic!("Can't crreate file: {:?}", error),
        },
        other_error => panic!("Can't open file")
    },
  };


  // Iterators 
  let mut arr_it: [i32; 4] = [1,2,3,4];

  // Below you can't change array values
  for val in arr_it.iter(){
    println!("{}",val);
  }

  // create iterator 
  let mut iter1 = arr_it.iter();
  println!("1st {:?}", iter1.next()); // Keep calling next to get the next value in iterator


// Closures are functions without a name AKA lambda functions 
  // let var_name = |parameters| ->return_type {BODY}

  let can_vote = |x: i32| -> bool{
        age >= 18
  };

  println!("Can vote {}", can_vote(8));

// Example 2 - Need to go over this later
//   let mut samp1: i32 = 5; 
//   let print_var: || -> () = || println!("samp1 = {}", samp1);

// Example 3 
fn use_func<T>(a: i32, b: i32, func: T) -> i32 
    where T: Fn(i32,i32) -> i32 {
        func(a,b)
}

let sum = |a:i32, b:i32| -> i32 {a+b};
let prod = |a:i32, b:i32| -> i32 {a*b};
println!("5 + 4 = {}", use_func(5, 5, sum));
println!("5 * 4 = {}", use_func(5, 5, prod));

