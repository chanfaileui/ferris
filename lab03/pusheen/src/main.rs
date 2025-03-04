fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // While an exclusive borrow is active, you cannot create additional exclusive/shared borrows of the same data.
    // This is to avoid two threads of execution both attempting to modify the original data (data races)
    let a = &mut vec;
    a.push(11);
    
    // The borrow a ends immediately after the last usage of a.push(11), allowing b to be created
    let b = &mut vec;
    b.push(12);

}
