fn main(){
	let a = 7; 
	let mut b = a; 
	b = b + 1;
	println!("a={}, b={}",a,b);
	// a=7, b=8
}

fn main(){
	let s1 = String::from("hello"); 
	let s2 = s1; 
	println!("s1 = {}, s2 = {}", s1, s2);

	// Wont compile
	// Why?
	// Rust has to figure out do you want 2 copies of the string?
	// OR do you want one copy and two variable pointing to the same string
}

fn main(){
	let s1 = String::from("hello"); 
	let s2 = s1.clone(); 
	println!("s1 = {}, s2 = {}", s1, s2);

	// Compiles since we have made a copy of s1 stored in s2
	// Each bit of data ex. "hello" has an owner 
	// And there can only be 1 owner 
	// Here s1 is the owner of data "hello"
}


fn main(){
	let s1 = String::from("hello"); 
	let s2 = &s1; 
	println!("s1 = {}, s2 = {}", s1, s2);

	// s1 is the owner of data "hello"
	// s2 is a reference that points to the same data
}


fn main(){
	lat s1 = String::from("hello"); 
	let s2 = &s1; 
	let s3 = &s2;
	println!("s1 = {}, s2 = {}", s1, s2);
	println!("s3 = {}, len = {}", s3, s3.len());

	// Can have multiple references
}


fn main(){
	lat s1 = String::from("hello"); 
	let s2 = &s1; 
	let s3 = &s2;
	println!("s1 = {}, s2 = {}", s1, s2);
	println!("s3 = {}, len = {}", s3, s3.len());

	// Can have multiple references
}


fn main(){
	// Mutable Borrow 
	let mut s1 = Str::from("hello"); 
	let s2     = &mut s1; // mutable borrow
	s2.push("!")
	println!("s2 = {}", s2);
	println!("s1 = {}", s1);

	// "hello" is borrowed by s2 
	// s2 printed then s1 the same
	// Reason: s2 goes out of scope so s1 has access to value "hello"
}


fn main(){
	// Mutable Borrow 
	let mut s1 = Str::from("hello"); 
	let s2     = &mut s1; // mutable borrow
	s2.push("!")
	println!("s2 = {}", s2);
	println!("s1 = {}", s1);
	s2.push("?");

	// Won't compile
	// Here s2 comes back into scope so s1 cannot use it because it is 
	// borrowed. Can't use s1 until s2 is out of scope.
}

// Functions that can change things 
fn append_dot(t: &mut String){
	t.push(".");
}

fn main(){
	let mut s = String::from("This is war");
	append_dot(&mut s); // s is borrowed. after function exec. s is returned
	println!("s with dot = {}",s);
}

// Dont forget to dereference with primitive types 

fn plus_one_by_ref(n: &mut i32){
	*n = *n +1; // Must use * to dereference primitives
	            // *n is the value of n
}

fn main(){
	let mut n: i32 = 19; 
	plus_one_by_ref(&mut n);
	println!("n = {}",n); // n = 20
}

fn main(){
	let mut words = vec![
		String::from("Hello"),
		String::from("Yellow"),
		String::from("Tree"),
		String::from("Rust"),
		String::from("Compiler"),
	];

	println!("{:?}", words);
	let t = &words[1];
	words[1] = words[2].clone();
	words[2] = *t; // Errrors out because we are borrowing t.
	println!("{:?}", words);


}













