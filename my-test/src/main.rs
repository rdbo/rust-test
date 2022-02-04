use std::fs::File;
use std::io::prelude::*;
use std::convert::From;
use std::fmt;
use rand::Rng;

mod mymod;

trait Animal {
	fn print_species(&self);
	fn print_age(&self);
}

fn main() {
	// this is a comment
	/*
	 * this is also
	 * a comment
	 */
	
	let mut i = 0;
	println!("Test: {}", i);

	loop {
		i += 1;

		if i > 5 && i < 10 {
			continue;
		} else if i > 20 {
			break;
		}

		println!("Loop I: {}", i);

	}

	i = 90;
	while i <= 100 {
		println!("While I: {}", i);
		i += 1;
	}

	let num_arr = 50..55;
	for j in num_arr {
		println!("For I: {}", j);
	}

	let color_vec = vec!["Red", "Green", "Blue"];
	for color in color_vec {
		println!("For Color: {}", color);
	}

	let word_list = vec!["Keyboard", "Mouse", "Computer"];
	for (index, word) in word_list.iter().enumerate() {
		println!("{}. {}", index, word);
	}

	enum ProcessState {
		Running,
		Waiting,
		Ready
	}

	let state = ProcessState::Running;
	match state {
		ProcessState::Running => println!("The process is running"),
		ProcessState::Waiting => println!("The process is waiting"),
		ProcessState::Ready => println!("The process is ready")
	}

	check_even_range(10, 20);

	let tup = (1, "Hello", 1.25, (10, 20, 30), false);

	println!("Tuple Second Value: {}", tup.1); // dafuq
	println!("Tuple: {:?}", tup);

	let shadow = "string";
	println!("Shadow: {}", shadow);
	let shadow = 0;
	println!("Shadow: {}", shadow);
	let shadow = true;
	println!("Shadow: {}", shadow);

	let myvar = "hello";
	let myref = &myvar;

	println!("My Reference: {}", myref);
	
	/*
	 * // does not work, myref is immutable
	 * let mut myval = 10;
	 * let myref = &myval;
	 * *myref += 1;
	 */

	/*
	 * // does not work, myval is immutable
	 * let myval = 10;
	 * let myref = &mut myval;
	 * *myref += 1;
	 */
	
	let mut myval = 10;
	let myref = &mut myval;
	*myref += 1;

	println!("My Value (Ref): {}", myref);
	println!("My Value: {}", myval); // doesn't work on older versions because myval has been borrowed or smth

	struct Person {
		name : String,
		age : u8
	}

	let person = Person { name: "John".to_string(), age : 27 };
	println!("Name: {}", person.name);
	println!("Age: {}", person.age);

	//Tuple Struct

	struct MyColor (u8, u8, u8);

	let mycolor = MyColor(255, 0, 0);
	println!("My Color: {} {} {}", mycolor.0, mycolor.1, mycolor.2);
	let mytest = MyTest { value: 10 };

	// does not work if parameter is not a reference (&MyTest), as 'mytest'
	// will be moved to a different scope
	my_test(&mytest);
	my_test(&mytest);

	let mut numbers : [u128;3] = [0;3]; // 3 uint128 values of 0
	numbers[0] = 0xFFFFFFFF;
	numbers[1] = 0xFFFFFFFFFFFFFFFF;
	numbers[2] = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;

	println!("Numbers Length: {}", numbers.len());
	for num in numbers.iter() {
		println!("Number: {}", num);
	}

	let mut file = File::open("test.txt").expect("Unable to open file!");
	let mut fdata = String::new();
	file.read_to_string(&mut fdata).expect("Unable to read file!");
	println!("Contents of 'test.txt': {}", fdata);

	// Implement functions for struct Person
	impl Person {
		fn print_info(&self) {
			println!("Name: {}", self.name);
			println!("Age: {}", self.age);
		}
	}

	let person = Person { name: "John Doe".to_string(), age : 69 };
	person.print_info();

	struct Dog {
		age : u16
	}

	// Implement Animal trait for Dog struct
	impl Animal for Dog {
		fn print_species(&self) {
			println!("The animal is a dog");
		}

		fn print_age(&self) {
			println!("The dog's age is: {}", self.age);
		}
	}

	let dog = Dog { age: 3 };
	print_animal_species(&dog);

	let exp0 = {
		let x = 10;
		let y = 20;
		x * y // no semicolon, this value will be assigned to exp0
	};
	println!("Expression 0: {}", exp0);

	let exp1 = {
		let x = 20;
		let y = 30;
		x * y; // there is semicolon, an empty tuple will be assigned
	};
	println!("Expression 1: {:?}", exp1);

	struct Number {
		num : i32
	}

	impl From<i32> for Number {
		fn from(num : i32) -> Self {
			Number { num: num }
		}
	}

	impl fmt::Display for Number { // 'automagically' implements ToString trait
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "Number({})", self.num) // return supressed by avoiding semicolon
		}
	}

	let number = Number::from(32);
	println!("Number Struct: {}", number);

	// what the fuck ???
	let rnum = rand::thread_rng().gen_range(0..100); // random number [0-100)
	let num = 
		if rnum < 50 {
			0
		} else {
			rnum * 10
		};
	
	println!("Random number: {}", rnum);
	println!("Number: {}", num);

	// break outter loops
	let mut n = 0;
	'outter: loop {
		loop {
			n += 1;
			print!("{} ", n);
			if n >= 10 {
				break 'outter;
			}
		}

		println!("This statement will not run");
	}
	println!("");

	// return value from loop
	n = 0;
	let j = loop {
		n += 1;
		if n >= 10 {
			break n; // return 'n' from loop
		}
	};
	println!("The value of J is: {}", j);

	print!("Yet another for loop: ");
	for n in 1..=5 { // loop from [1, 5]
		print!("{} ", n);
	}
	println!("");

	// match
	let rnum = rand::thread_rng().gen_range(1..15);
	println!("Random Number: {}", rnum);
	match rnum {
		1 => println!("The number is one"),
		2 | 3 | 4 => println!("The number is 2 or 3 or 4"),
		5..=10 => println!("The number is in the range [5, 15]"),
		_ => println!("The number is greater than 15")
	}

	// destructure tuples
	let tuple = (1, 5, 10);
	match tuple {
		(1, 1, 1) => println!("The tuple is full of ones"),
		(1, 5, ..) => println!("The tuple starts with 1, 5"),
		_ => println!("The tuple is unknown")
	}

	// destructure pointers
	let var = 10;
	let var_ref = &var;
	match var_ref {
		&val => println!("Destructured reference: {}", val)
	}

	match *var_ref {
		val => println!("Value from dereference: {}", val)
	}

	let mut var = Some(Number::from(32));
	match var {
		// ref used to prevent var from being consumed
		Some(ref mut var_newref) => {
			println!("New reference: {}", var_newref);
			(*var_newref).num += 10;
			var_newref.num *= 10;
		},
		_ => println!("None")
	}
	println!("Var is still accessible: {}", var.unwrap());

	// destructure structs
	struct Person2 {
		id : char,
		age : u8
	}
	let person = Person2 { id: 'B', age: 45 };
	match person {
		Person2 { id: 'A', age: 24 } => println!("Hello, Alice!"),
		Person2 { id: 'B', .. } => println!("Welcome, Bob"),
		_ => println!("User unknown")
	}

	let tuple = (1, 2, 3);
	match tuple {
		(1, y, 3) if y > 5 => println!("Tuple: (1, y, 3), y > 5"),
		(2, y, 5) if y <= 0 => println!("Tuple: (2, y, 5), y <= 0"),
		(1, y, 3) if y > 0 && y < 3 => println!("Tuple: (1, y, 3), 0 < y < 3"),
		_ => println!("Tuple unidentified")
	}

	let number = Some(10);
	if let Some(i) = number {
		println!("Value of I: {}", i);
	}

	let inval : Option<i32> = None;
	// pattern matching in if
	if let Some(j) = inval {
		println!("Variable is not None");
	} else {
		println!("Variable is None");
	}

	if inval != None {
		println!("Valid");
	} else {
		println!("Invalid");
	}

	let number = 10;
	if let 9 = number {
		println!("Number is 9");
	} else if let 10 = number {
		println!("Number is 10");
	}

	let mut num = Some(0);
	while let Some(i) = num {
		if i >= 10 {
			break;
		}

		println!("Value of I: {}", i);
		num = Some(i + 1);
	}

	let closure_sum = |x : i32, y : i32| -> i32 { x + y };
	println!("10 + 20 = {}", closure_sum(10, 20));

	let mystr = String::from("This is a string");
	let closure_print = || println!("My String: {}", mystr);
	closure_print(); // will not move 'mystr'
	println!("My String Again: {}", mystr);

	let vec = vec![10, 20, 30, 40, 50];
	let printvec = move || {
		println!("Vector: {:?}", vec);
	};

	printvec();
	printvec();

	let get_square = |i : i32| i * i;
	let result = call_closure(get_square, 16);
	println!("Call Closure Result: {:?}", result);

	let my_fn = create_fn();
	let mut my_fnmut = create_fnmut();
	let my_fnonce = create_fnonce();

	my_fn();
	my_fnmut();
	my_fnonce();

	mymod::my_pub_function();
	mymod::mysubmod::my_pub_function();
	let mut myobj = mymod::MyStruct::new();
	myobj.myint = 69;
	// myobj.myprivint = 420; // private field cannot be accessed
	myobj.set_private(420);
	println!("My Int: {}", myobj.myint);
	// println!("My Private Int: {}", myobj.myprivint);
	myobj.print_private();

	use this_function_has_a_very_long_name as long_func;
	long_func();

	struct MyGeneric<T> (T);
	let igen : MyGeneric<i32> = MyGeneric(10);
	let cgen = MyGeneric('c');
	let bgen = MyGeneric(true);
	println!("Value of igen: {:?}", igen.0);
	println!("Value of cgen: {:?}", cgen.0);
	println!("Value of bgen: {:?}", bgen.0);
}

fn this_function_has_a_very_long_name() {
	println!("Function with long name");
}

fn check_even_range(n1 : u32, n2 : u32) {
	for i in n1 .. n2 {
		println!("Is {} even? {}", i, is_even(i));
	}
}

fn is_even(n : u32) -> bool {
	return n & (1 << 0) == 0;
}

struct MyTest {
	value : u32
}

fn my_test(test : &MyTest) {
	println!("Test: {}", test.value);
}

fn print_animal_species(animal : &dyn Animal) {
	animal.print_species();
}

fn call_closure<F>(f : F, arg : i32) -> i32 where F : Fn(i32) -> i32 {
	f(arg)
}

fn create_fn() -> impl Fn() {
	move || println!("This is an Fn function")
}

fn create_fnmut() -> impl FnMut() {
	move || println!("This is an FnMut function")
}

fn create_fnonce() -> impl FnOnce() {
	move || println!("This is an FnOnce function")
}
