// Rustlang Functions
fn main() {
	let seven: i32 = add_one(6);
	let five: i32 = add(100, 400);

	println!("add_one is a pure function");
	println!("Result: {}", seven);
	println!("Result: {}", five);

	function_pointers();

	divergent();
}


// Functions
// All argument types must be declared
// Specify a return type with an arrow ->
// Best practice is to use return expressions

fn add_one(x: i32) -> i32 {
    // We leave off the semi-colon because this is a return expression
    x + 1
}

fn add(x: i32, y: i32) -> i32 {
	/* 
		Early Return is poor style
		return x;
	*/

	x + y
}


// Function Pointers
// Variable bindings that point to functions
// Wrapping in a function because let needs to be declared inside a fn

fn function_pointers() {
	// If we are declaring literal string types, needs to use the static sytax
	fn stringbean(x: &'static str) -> &'static str {
		return x;
	}

	let my_func: fn(&'static str) -> &'static str = stringbean;

	println!("The bean of the year award goes to.. {}!", my_func("String Beans"));
}


// Diverging functions
// By giving a return type of !, we cause the program to crash
// This can be useful in throwing error messages

fn divergent() -> ! {
	panic!("ALERT! THROWING ERROR! This function never returns..");
}