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

	
}

fn check_even_range(n1 : u32, n2 : u32) {
	for i in n1..n2 {
		println!("Is {} even? {}", i, is_even(i));
	}
}

fn is_even(n : u32) -> bool {
	return n & (1 << 0) == 0;
}
