fn main() {    let mut v = vec![1, 2, 3];    let mut iter = v.iter_mut();    let first = iter.next().unwrap();    *first = 10;    println!( "First element: {:?}", v);}