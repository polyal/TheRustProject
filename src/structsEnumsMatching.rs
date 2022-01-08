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

/*** scope_eg ***/

fn main() 
{

}