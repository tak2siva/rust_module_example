use nested_modules::mod_b;

pub fn foo() {
	println!("Im in mod_a");
}

pub fn bar() {
	println!("calling mod_b from mod_a");
	mod_b::foo();
	println!("");
}