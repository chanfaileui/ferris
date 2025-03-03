use std::collections::{HashMap, LinkedList, VecDeque};

const MAX_ITER: i32 = 300000;

fn main() {
    // Vectors
    vec_operations();

    // VecDeque
    vec_deque_operations();

    // TODO: your code here, for linked list insertions
    linked_list_operations();

    // TODO: your code here, for hashmap insertions
    hashmap_operations();

    // TODO: your text explanation to the questions in the spec

    // * Which collection type was the fastest for adding and removing elements?
    // * Why do you think this was the case?
    // Vec and Vecdeque were the fastest for adding and removing elements. That’s because Vectors store elements contiguously
    // which enables better cache locality and reduces memory fragmentation compared to other collections.

    // * Is there any significant difference between Vec and VecDeque deletion?
    // * If so, why? If not, why not?
    // Yes, Vec is dramatically slower (s vs ms). Vec is O(n) because it requires shifting all remaining elements leftward,
    // whereas VecDeque removal from the front is O(1) because it simply adjusts internal pointers in its ring buffer implementation.

    // * When would you consider using VecDeque over Vec?
    // VecDeque over Vec when implementing queue-like data structures (FIFO), when elements need to be efficiently added or removed from both ends

    // * When would you consider using LinkedList over Vec?
    // To be honest you would rarely consider LinkedList over Vec in Rust. The theoretical case would be appending frequently
    // and you can't afford occasional large reallocations (cannot tolerate amortisation),
    // and when you need to frequently splice or append lists together.

    // * Did the results suprise you? Why or why not?.
    // I thought linked list would be the worst at everything but vec’s removal surprised me at how bad it was.
    // I also though hash map would perform better. Even though they have O(1) asymptotic thoerticacal time complexity,
    // there’s overhead of setting up the hash map which makes it perform worse practically for smaller datasets.
}

/// measure the insertion and removal
/// operations of a vector
fn vec_operations() {
    let mut vec = Vec::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec.push(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Vector ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec.remove(0);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

/// measure the insertion and removal
/// operations of a VecDeque
fn vec_deque_operations() {
    let mut vec_deque = VecDeque::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec_deque.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== VecDeque ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec_deque.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

fn linked_list_operations() {
    let mut list = LinkedList::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        list.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Linked List ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        list.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

fn hashmap_operations() {
    let mut hashmap = HashMap::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hashmap.insert(i, i);
    }
    let time_end = std::time::Instant::now();

    println!("==== HashMap ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hashmap.remove(&i);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}
