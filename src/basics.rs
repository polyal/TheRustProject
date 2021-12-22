// https://stevedonovan.github.io/rust-gentle-intro/readme.html

// #[allow(dead_code)]
// supresses "warning: function is never used: '' "


/*** Hello, World! ***/
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

/*** Looping and Ifing ***/
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

/*** Adding Things Up ***/
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

/*** Function Types are Explicit ***/
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

/*** Learning Where to Find the Ropes ***/
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

/*** Arrays and Slices ***/
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
// its more like a C++ reference in some ways (more strict)
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

/*** Slicing and Dicing ***/
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

/*** Optional Values ***/
#[allow(dead_code)]
fn slice_get_eg()
{
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);

    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());

    let maybe_last = slice.get(5);
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap()
    } else {
        -1
    };
    println!("last value {}", last);

    let last = *slice.get(5).unwrap_or(&-1);
    println!("last value {}", last);
}

/*** Vectors ***/
// only difference between vectors and arrays is that
// vectors are allocated dynamically while arrays statically
fn dump_i32_slice(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

#[allow(dead_code)]
fn vector_eg()
{
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];  // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);

    dump_i32_slice(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
}

/*** Iterators ***/
#[allow(dead_code)]
fn iterator_eg()
{
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

#[allow(dead_code)]
fn array_iterator()
{
    // this way doesn't work in older versions?
    let arr = [10, 20, 30];
    for i in arr
    {
        println!("{}", i);
    }

    // new way doesnt require the '.iter()' as seen above
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators...
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }

    // above examples are more efficien than
    // for i in 0..slice.len() {}
}

#[allow(dead_code)]
fn iterator_sum()
{
    let sum: i32  = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);
}

#[allow(dead_code)]
fn windows_and_chunks()
{
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    // a moving windows
    for s in slice.windows(2) {
        println!("window {:?}", s);
    }

    // discreet chunks
    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
    }
}

/*** More about vectors... ***/
#[allow(dead_code)]
fn extend_and_pop()
{
    let mut v1 = vec![10, 20, 30, 40];
    println!("Original Vec: {:?}", v1);
    v1.pop();
    println!("After pop: {:?}", v1);

    let mut v2 = Vec::new();
    println!("Original Vec: {:?}", v2);
    v2.push(10);
    v2.push(20);
    v2.push(30);
    println!("After push: {:?}", v2);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    println!("After extend: {:?}", v2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);

}

#[allow(dead_code)]
fn clone_sort_dedup() 
{
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    let v2 = v1.clone();
    v1.sort();
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
    assert_eq!(v2, &[1, 10, 5, 1, 2, 11, 2, 40]);
    println!("v1 {:?}", v1);
    println!("v2 {:?}", v2);
}

/*** Strings ***/
// keep in mind strings are just bytes [u8] like in c,
// but here they have to be valid UTF-8 text
// ** they are not 'char's though
fn dump_str_slice(s: &str) 
{
    println!("str '{}'", s);
}

#[allow(dead_code)]
fn strings_eg() 
{
    let text = "hello dolly";  // the string slice (static)
    let s = text.to_string();  // it's now an allocated string

    dump_str_slice(text);
    dump_str_slice(&s);
}

#[allow(dead_code)]
fn push_and_pop_strings() 
{
    let mut s = String::new();
    
    // initially empty!
    println!("Initial str {:?}", s);
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for `push_str`
    println!("After push/'+=' {:?}", s);
    
    // remove the last char
    s.pop();
    println!("After pop {:?}", s);
    assert_eq!(s, "Hello World");
}

fn array_to_str(arr: &[i32]) -> String 
{
    let mut res = '['.to_string();
    for v in arr 
    {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

#[allow(dead_code)]
fn format_macro_eg() 
{
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);
    println!("{:?}", res);
    assert_eq!(res, "hello [10,20,30]");
}

#[allow(dead_code)]
fn strings_slice_notation()
{
    let text = "static";
    let string = "dynamic".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("slices {:?} {:?}", text_s, string_s);
}

#[allow(dead_code)]
fn utf_8_eg()
{
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() 
    {
        print!("'{}' ", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() 
    {
        // print 'п' and onwards
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
}

#[allow(dead_code)]
fn more_egs()
{
    // let s = "¡";
    // println!("{}", &s[0..1]); // <-- bad, first byte of a multibyte character

    let text = "the red fox and the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("text: {}", text);
    println!("collect words: {:?}", words);

    let mut words = Vec::new();
    words.extend(text.split_whitespace());
    println!("extend words: {:?}", words);

    // filter takes a closure (C++ lanbda)
    let stripped: String = text.chars().filter(|ch| ! ch.is_whitespace()).collect();
    println!("filter words: {:?}", stripped);
}

/*** Interlude: Getting Command Line Arguments ***/

fn main() 
{
	more_egs();
}
