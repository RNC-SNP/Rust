fn main() {
	// define constant variable:
	let lang = "Rust";
	// print string with variable:
	println!("Hello, {}!", lang);
	
	// define mutable variable: 
	let mut count = 0;
	while count < 10 {
		println!("count: {}", count);
		count += 1;// '++' operation is not supported.
	}

	// define type:
	type MyType = int;

	// define variable with if-else expression:
	let x =
		if is_ten(10) {
			1
		} else {
			0
		};
	println!("x = {}", x);
}

// define function:
fn is_ten(x: int) -> bool {
	// No need for a return statement.
	// The result of the expression is used as the return value. 
	x == 10
}
