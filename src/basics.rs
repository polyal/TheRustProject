// https://stevedonovan.github.io/rust-gentle-intro/readme.html

// #[allow(dead_code)]
// supresses "warning: function is never used: '' "


/**** Hello, World! ****/
#[allow(dead_code)]
fn hello_world()
{
	println!("Hello, World!");
}

#[allow(dead_code)]
fn let_eg()
{
	let answer = 42;
	println!("Hello, {}", answer);
	assert_eq!(answer,42);
}

#[allow(dead_code)]
fn assert_eg()
{
	let answer = 42;
	assert_eq!(answer,40);
}

/**** Looping and Ifing ****/
#[allow(dead_code)]
fn loop_eg()
{
	for i in 0..5
	{
		println!("Hello, {}", i);
	}
}

#[allow(dead_code)]
fn if_eg()
{
	for i in 0..5
	{
		if i % 2 == 0
		{
			println!("even {}", i);	
		}
		else 
		{
			println!("odd {}", i);		
		}
		
	}
}

// Almost everything in Rust is an expression and has a value,
// even the 'if' statements, therefore they can be assigned to
// a var and executed something like a fn pointer or lambda in C++
#[allow(dead_code)]
fn expr_eg()
{
	for i in 0..5
	{
		let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
		
	}
}

/**** Adding Things Up ****/
#[allow(dead_code)]
fn mut_eg()
{
	let mut sum = 0;
    for i in 0..5 
    {
        sum += i;
    }
    println!("sum is {}", sum);
}

#[allow(dead_code)]
fn cast_and_traits_eg()
{
	let mut sum = 0.0;
    for i in 0..5 
    {
        sum += i as f64;
    }
    println!("sum is {}", sum);
}

/**** Function Types are Explicit ****/
#[allow(dead_code)]
fn sqr(x: f64) -> f64
{
	let y = x * x;
    println!("{}^2 = {}", x, y);
    return y;
}

// Another expression example.  Here, the function itself is an
// expression and therefore doesn't need a semicolom.  Because,
// x*x is the last expression in the function, it is what will be
// returned.  If we were to add a semicolon after x*x, we would get
// an error because then we'd be returning '()' (nothing), instead of
// a f64.  And nothing is not f64
#[allow(dead_code)]
fn sqr_expr(x: f64) -> f64 
{
    x * x
}

// absolute value of a floating-point number
#[allow(dead_code)]
fn abs(x: f64) -> f64 
{
    if x > 0.0 
    {
        x
    } 
    else 
    {
        -x
    }
}

// ensure the number always falls in the given range
#[allow(dead_code)]
fn clamp(x: f64, x1: f64, x2: f64) -> f64 
{
    if x < x1 
    {
        x1
    } 
    else if x > x2 
    {
        x2
    } 
    else 
    {
        x
    }
}

// recursive example
#[allow(dead_code)]
fn factorial(n: u64) -> u64 
{
    if n == 0 
    {
        1
    } 
    else 
    {
        n * factorial(n-1)
    }
}

#[allow(dead_code)]
fn by_ref(x: &i32) -> i32
{
    *x + 1
}

// references of literals(?) can also be passed
#[allow(dead_code)]
fn by_ref_eg()
{
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1,res2);
}

#[allow(dead_code)]
fn modifies(x: &mut f64) 
{
    *x = 1.0;
}

// don't forget to pass the '&mut' in to 'pass by reference'
#[allow(dead_code)]
fn ref_eg() 
{
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
}

// also valid to name a type during a declaration
//let bigint: i64 = 0;

/**** Learning Where to Find the Ropes ****/
// seems like everything is a class/object kinda like in Java.
// look at x.cos()
// use std::f64::consts; isnt needed because Rust makes some basic functionality
// visible without having to include it
#[allow(dead_code)]
fn cosine_eg()
{
	let x = 2.0 * std::f64::consts::PI;
    let abs_difference = (x.cos() - 1.0).abs();
    assert!(abs_difference < 1e-10);
    println!("abs_difference {}", abs_difference);
}

/**** Arrays and Slices ****/
#[allow(dead_code)]
fn array_eg()
{
	let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i,arr[i]);
    }
    println!("length {}", arr.len());
}

// returns 'index out of bounds' a compile time
/*#[allow(dead_code)]
fn array_eg2()
{
	let arr = [10, 20, 30, 40];
    println!("{}", arr[4]);
}*/

// this is a slice, when an array is passed by ref
// kinda like a c pointer but more strict
#[allow(dead_code)]
fn sum(values: &[i32]) -> i32 
{
    let mut res = 0;
    for i in 0..values.len() 
    {
        res += values[i]
    }
    res
}

#[allow(dead_code)]
fn sum_caller()
{
	let arr = [10,20,30,40];
    // look at that &
    let res = sum(&arr);
    println!("sum {}", res);
}

/**** Slicing and Dicing ****/
#[allow(dead_code)]
fn printing_arrays()
{
	let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
}

// gives an error because the float array isnt of type '()'
// let var: () = [1.1, 1.2];

#[allow(dead_code)]
fn range_and_borrow()
{
	let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];  // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}

/**** Optional Values ****/

fn main() 
{
	range_and_borrow();
}
