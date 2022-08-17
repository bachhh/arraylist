fn main() {
    println!("Hello, world!");
}

pub struct ArrayList {
    array: Vec<Option<u32>>,
    // call to pop() will return list[head]
    head: usize,
    // call to push() will put element into list[tail]
    // tail >= (head + length) % capacity,
    // if array is densely packed, tail == head + length
    // if array is empty, head == tail
    tail: usize,

    length: usize,
}
