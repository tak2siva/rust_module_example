mod single_module;
mod single_mod_in_folder;

mod multiple_modules;

mod nested_modules;

fn main() {
	single_module::foo();
	single_mod_in_folder::foo();
	
	println!("---");
	multiple_modules::mod1::foo();
	multiple_modules::mod2::foo();

	println!("---");
	nested_modules::mod_a::bar();

	println!("---");
	nested_modules::mod_b::bar();
}