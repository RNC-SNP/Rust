fn main() {
	// define constant variable:
	let lang = "Rust";

	// print string with variable:
	println!("Hello, {}!", lang);

	// define type:
	type MyType = int;

	use_if_else();

	use_match();

	use_while();

	use_loop();

	use_for_in();
}

// define function:
fn is_ten(x: int) -> bool {
	// No need for a return statement.
	// The result of the expression is used as the return value. 
	x == 10
}

fn use_if_else() {
	// define variable with if-else expression:
	let x =
		if is_ten(10) {
			1i
		} else {
			0i
		};
	println!("x = {}", x);
}

fn use_match() {
	let month = 7i;
	// like 'switch/case' in other languages.
	// like golang, there's no 'falling through'.
	let max_day =
		match month {
			2 => 28i,
			4 | 6 | 9 | 11 => 30i,
			_ => 31i
		};
	println!("The max day of month({}) is {};", month, max_day);
}

fn use_while() {
	// define mutable variable: 
	let mut count = 0i; // 'count' is an 'int'
	while count < 10 {
		println!("count: {}", count);
		count += 1;// '++' operation is not supported.
	}
}

fn use_loop() {
	let mut x = 100u;
	//like 'while(true)'
	loop {
		x -= 10;
		println!("x: {}", x);
		if x <= 0 {
			break;
		}
	}
}

fn use_for_in() {
	for y in range(0u, 10) {
		println!("y: {}", y);
	}
	let str = "RincLiu";
	for chr in str.chars() {
		println!("{}", chr);
	}
}