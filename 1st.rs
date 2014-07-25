fn main() {
	// define constant variable:
	let lang = "Rust";
	// print string with variable:
	println!("Hello, {}!", lang);
	
	// define mutable variable: 
	let mut count = 0i; // 'count' is an 'int'
	while count < 10 {
		println!("count: {}", count);
		count += 1;// '++' operation is not supported.
	}

	// define type:
	type MyType = int;

	// define variable with if-else expression:
	let x =
		if is_ten(10) {
			1i
		} else {
			0i
		};
	println!("x = {}", x);

	let month = 7;
	println!("The max day of {} is {}", month, max_day(month))
}

// define function:
fn is_ten(x: int) -> bool {
	// No need for a return statement.
	// The result of the expression is used as the return value. 
	x == 10
}

// Pattern matching:
fn max_day(month: int) -> int {
	// like 'switch/case' in other languages
	match month {
		2 => 28,
		4 | 6 | 9 | 11 => 30,
		_ => 31
	}
}
