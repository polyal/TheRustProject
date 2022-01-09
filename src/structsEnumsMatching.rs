// https://stevedonovan.github.io/rust-gentle-intro/readme.html
// https://stevedonovan.github.io/rust-gentle-intro/2-structs-enums-lifetimes.html

/*** Rust likes to Move It, Move It ***/
/************************************************
#[allow(dead_code)]
fn move_eg()
{
    let s1 = "hello_dolly".to_string();
    let s2 = s1;
    println!("s1 {}", s1);

}

// creates this error because string is not a moveable type in rust
error[E0382]: borrow of moved value: `s1`
 --> structsEnumsMatching.rs:9:23
  |
7 |     let s1 = "hello_dolly".to_string();
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
8 |     let s2 = s1;
  |              -- value moved here
9 |     println!("s1 {}", s1);
  |                       ^^ value borrowed here after move

error: aborting due to previous error; 1 warning emitted
************************************************/

/************************************************
fn dump(s: String)
{
    println!("{}", s);
}

// same thing happens here, s1 gets moved to the 'dump' function and no longer exists after the call
fn move_eg2()
{
    let s1 = "hello_dolly".to_string();
    dump(s1);
    println!("{}", s1);
}

rror[E0382]: borrow of moved value: `s1`
  --> structsEnumsMatching.rs:37:20
   |
35 |     let s1 = "hello_dolly".to_string();
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
36 |     dump(s1);
   |          -- value moved here
37 |     println!("{}", s1);
   |                    ^^ value borrowed here after move

error: aborting due to previous error
************************************************/

fn dump(s: &String)
{
    println!("{}", s);
}

// use reference to solve above error
#[allow(dead_code)]
fn move_eg3()
{
    let s1 = "hello_dolly".to_string();
    dump(&s1);
    dump(&"hello_dolly".to_string()); // or for literals, but its ugly
    println!("{}", s1);
}

/*** Scope of Variables ***/
#[allow(dead_code)]
fn scope_eg()
{
    let a = 10;
    let b = "hello";
    {
        let _c = "hello".to_string();
        // a,b and c are visible
    }
    // the string c is dropped
    // a,b are visible
    for _i in 0..a {
        let b = &b[1..];
        println!("b: {}", b);
        // original b is no longer visible - it is shadowed.
    }
    // the slice b is dropped
    // i is _not_ visible!
}

/************************************************
#[allow(dead_code)]
fn scope_ref_eg()
{
    let s1 = "hello_dolly".to_string();
    let mut rs1 = &s1;
    {
        let tmp = "hello_dolly".to_string();
        rs1 = &tmp;
    }
    println!("rs1 {}", rs1);
}

// in rust, expired references are compile errors not run time errors
error[E0597]: `tmp` does not live long enough
   --> structsEnumsMatching.rs:99:15
    |
99  |         rs1 = &tmp;
    |               ^^^^ borrowed value does not live long enough
100 |     }
    |     - `tmp` dropped here while still borrowed
101 |     println!("rs1 {}", rs1);
    |                        --- borrow later used here

error: aborting due to previous error; 1 warning emitted
************************************************/

/*** Tuples ***/
fn add_mul(x: f64, y: f64) -> (f64, f64)
{
    (x+y, x*y)
}

#[allow(dead_code)]
fn tuples_eg()
{
    let t = add_mul(2.0, 10.0);

    // can debug print
    println!("{:?}", t);

    // can index the values
    println!("add {} mul {}", t.0, t.1);

    // can extract values
    let (add, mul) = t;
    println!("add {} mul {}", add, mul);


    // tuple with different types
    let t2 = ("hello", 5, 'c');
    assert_eq!(t2.0, "hello");
    assert_eq!(t2.1, 5);
    assert_eq!(t2.2, 'c');


    // enumerate eg
    for t in ["zero", "one", "two"].iter().enumerate()
    {
        println!("{} {}", t.0, t.1);
    }

    // zip eg
    let names = ["ten", "hunred", "thousand"];
    let nums = [10, 100, 1000];
    for p in names.iter().zip(nums.iter())
    {
        println!("{} {}", p.0, p.1);
    }
}

/*** Structs ***/
// to allow debug printing '{:?}'
// we replace this generated def with one below
//#[derive(Debug)]
struct Person
{
    first_name: String,
    last_name: String
}

impl Person 
{
    fn new(first: &str, name: &str) -> Person
    {
        Person
        {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    fn full_name(&self) -> String
    {
        format!("{} {}", self.first_name, self.last_name)
    }

    // 'Self' refers to the 'Person' type itself
    fn copy(&self) -> Self
    {
        Self::new(&self.first_name, &self.last_name)
    }

    fn set_first_name(&mut self, name: &str)
    {
        self.first_name = name.to_string();
    }

    // self (no '&') will cause data to be moved to tuple
    fn to_tuple(self) -> (String, String)
    {
        (self.first_name, self.last_name)
    }
}

#[allow(dead_code)]
fn struct_eg()
{
    let p = Person
    {
        first_name: "John".to_string(),
        last_name: "Smith".to_string()
    };
    println!("person {} {}", p.first_name, p.last_name);

    // cosntructor eg
    let p2 = Person::new("John", "Smith");
    println!("person {} {}", p2.first_name, p2.last_name);

    // format eg
    println!("fullname {}", p2.full_name());

    // copy and set
    let mut p3 = p2.copy();
    p3.set_first_name("Joe");
    println!("fullname1 {} -> fullname2 {}", p2.full_name(), p3.full_name());

    // to_tuple eg - keep in mind this is a move operation
    let pt = p3.to_tuple();
    println!("first {} last {}", pt.0, pt.1);

    // debug eg
    let p4 = Person::new("Jane", "Doe");
    println!("person {:?}", p4);
}

/*** Lifetimes Start to Bite ***/
#[derive(Debug)]
struct A
{
    // let rust know were referencing a string literal
    // otherwise known as a string with a 'static' lifetime
    s: &'static str
}

#[allow(dead_code)]
fn struct_with_slice()
{
    let a = A {s: "hello dammit"};
    println!("{:?}", a);
}

// example where we return a 'static' string
fn how(i: u32) -> &'static str 
{
    match i {
    0 => "none",
    1 => "one",
    _ => "many"
    }
}

#[allow(dead_code)]
fn static_return()
{
    println!("{}", how(123))
}

#[derive(Debug)]
struct B <'a>
{
    // this means that 's' will live at least as long as
    // what it is referencing
    s: &'a str
}

#[allow(dead_code)]
fn struct_ref_lifetime()
{
    let s = "I'm a little string".to_string();
    let b = B {s: &s};
    println!("{:?}", b);
}

/************************************************
fn makes_B() -> B
{
    let s = "I'm a little string".to_string();
    B {s: &s}
}

// rust wont allow this because the return type of 'make_B()'
// needs to show that its a lifetime variable
error[E0106]: missing lifetime specifier
   --> structsEnumsMatching.rs:285:17
    |
285 | fn makes_B() -> B
    |                 ^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
    |
285 | fn makes_B() -> B<'static>
    |                 ^^^^^^^^^^

error: aborting due to previous error
************************************************/

/************************************************
fn makes_B() -> B<'static>
{
    let s = "I'm a little string".to_string();
    B {s: &s}
}

// fixed the above error, but still wont work because 's' will outlive 'b'
error[E0515]: cannot return value referencing local variable `s`
   --> structsEnumsMatching.rs:312:5
    |
312 |     B {s: &s}
    |     ^^^^^^--^
    |     |     |
    |     |     `s` is borrowed here
    |     returns a value referencing data owned by the current function

error: aborting due to previous error
************************************************/

/*** Traits ***/
trait Show
{
    fn show(&self) -> String;
}

impl Show for i32
{
    fn show (&self) -> String
    {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64
{
    fn show (&self) -> String
    {
        format!("eight-bte float {}", self)
    }
}

#[allow(dead_code)]
fn trait_eg()
{
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);
}

use std::fmt;

// 'Person' defined at ln167
impl fmt::Debug for Person
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.full_name())
    }
}

#[allow(dead_code)]
fn fmt_eg()
{
    let p = Person::new("John", "Buck");
    println!("{:?}", p);
}

/*** Example: iterator over floating-point range ***/
struct FRange
{
    val: f64,
    end: f64,
    incr: f64
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange
{
    FRange {val: x1, end: x2, incr: skip}
}

impl Iterator for FRange 
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item>
    {
        let res = self.val;
        if res >= self.end
        {
            None
        }
        else
        {
            self.val += self.incr;
            Some(res)
        }
    }
}

#[allow(dead_code)]
fn iterator_trait_eg()
{
    for x in range(0.0, 1.0, 0.1)
    {
        println!("{:.1}", x);
    }
}

// without ':1' result is going to be messy because some decimals cannot be represented well
/***********************************
0                          vs    0.0
0.1                              0.1
0.2                              0.2
0.30000000000000004              0.3
0.4                              0.4
0.5                              0.5 
0.6                              0.6
0.7                              0.7
0.7999999999999999               0.8
0.8999999999999999               0.9
0.9999999999999999               1.0
***********************************/

/*** Generic Functions ***/
// print out any type that implements 'Debug'
fn dump_t<T> (value: &T)
where T: std::fmt::Debug
{
    println!("value is {:?}", value);
}

#[allow(dead_code)]
fn generic_eg()
{
    let n = 42;
    dump_t(&n);
}

// These functions are called monomorphic, in constrast to polymorphic. The body of 
// the function is compiled separately for each unique type. With polymorphic functions, 
// the same machine code works with each matching type, dynamically dispatching the correct method.
fn sqr<T> (x: T) -> T::Output
where T: std::ops::Mul + Copy
{
    x*x
}

fn generic_eg2()
{
    let num = 10.0;
    let res = sqr(num);
    println!("{} * {} = {}", num, num, res);
}

/*** Simple Enums ***/

fn main() 
{
    generic_eg2();
}
