fn main() {
    let mut v = vec![1, 2, 3];

    // Solution 1: Using indexing
    v[0] = 10; 
    println!("First element (using indexing): {:?}", v);

    // Solution 2: Consuming and replacing using iterator
    let mut v2 = vec![1, 2, 3];
    let first_index = v2.iter().position(|&x| x == 1).unwrap();
    v2[first_index] = 10;
    println!("First element (consuming iterator): {:?}", v2);
} 