// pub mod mymod { // implicit mod
pub fn my_pub_function() { // public function
	println!("Public function from 'mymod'");
	my_private_function();
	mysubmod::my_super_function();
}

pub fn my_ambiguous_function() {
	println!("Ambiguous function from 'mymod'");
}

pub(self) fn my_private_function() { // private, same as not using pub at all
	println!("Private function from 'mymod'");
}

pub mod mysubmod {
	pub fn my_pub_function() { // public function
		println!("Public function from 'mysubmod'");
		self::my_ambiguous_function();
		super::my_ambiguous_function();
	}
	pub fn my_ambiguous_function() {
		println!("Ambiguous function from 'mysubmod'");
	}
	pub(super) fn my_super_function() { // only visible in parent module
		println!("Super function from 'mysubmod'");
	}
}

type myint = i32;

pub struct MyStruct {
	pub myint : myint,
	myprivint : myint
}

impl MyStruct {
	pub fn new() -> Self {
		return MyStruct { myint: 0, myprivint: 0 };
	}
	pub fn set_private(&mut self, val : myint) {
		self.myprivint = val;
	}
	pub fn print_private(&self) {
		println!("My Private Int: {}", self.myprivint);
	}
}

// }
