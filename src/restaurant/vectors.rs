mod vectors{


	pub fn vec_example(){
		// Way 1 - Declare a new vector
		// vectors are stored in the heap
		let v: Vec<i32> = Vec::new(); // <i32> is a generic 
		println!("{:?}", v);

		// Way 2 - Declare vector 
		// Using a macro to delcare a vector
		// No need to sepcify type as its done via type inferencing
		let v1 = vec![1,2,3];
		println!("{:?}", v1);


		let mut v2 = Vec::new(); // Declaring mutable vector 
		v2.push(5);
		v2.push(6);
		v2.push(7);
		v2.push(8);

		let third: &i32 = &v[2];
		println!("This third element is {}", third);

		match v2.get(2){
			Some(third) => println!("The third elenent is {}", third),
			None => println!("There is no third element")
		} 


		// Index that does not exists
		let does_not_exist = &v[100]; // panics 
		let does_not_exist = v.get(100) // returns None

		// Borrowing 

		// ====== Example 1 ======/
		// Wont work since v2 might need to reallocated in the heap if enough size is 
		// not allocated thus changing the memory addresses. so Rust will stop us here
		let first = &v2[0];
		v2.push(6);
		println!("The first element is {}", first);

		// Looping 

		// ====== Example 1 ======/
		let v3 = vec![100,32,57];
		for i in &v3 {
			println!("{}", i);
		}

		// ====== Example 2 ======/
		// Changing elements in a mutable vector
		let mut v4 = vec![100,32,57];

		for i in &mut v {
			*i += 50;
		}

		// Storing other objects 

		enum SpreadsheetCell {
			Int(i32),
			Float(f64),
			Text(String)
		}

		let mut row = vec![
			SpreadsheetCell::Int(3),
			SpreadsheetCell::Text(String::from("blue")),
			SpreadsheetCell::Float(10.12),
		];

		for (index ,element) in row.iter().enumerate(){
			println!("index {}, element: {}", index, element);
		}




	}

}