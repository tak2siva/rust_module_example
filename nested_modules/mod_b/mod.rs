use nested_modules::mod_a;

pub fn foo() {
	println!("Im in mod_b");
}

pub fn bar() {
	println!("calling mod_a from mod_b");
	mod_a::foo();
	println!("");
}