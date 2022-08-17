fn main() {
    println!("Hello, world!");
}

const MIN_DENSITY: f64 = 0.5;

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

impl ArrayList {
    pub fn new() -> ArrayList {
        ArrayList {
            array: Vec::with_capacity(16),
            head: 0,
            tail: 0,
            length: 0,
        }
    }

    pub fn push(&mut self, entry: u32) {
        self.array[self.head] = Some(entry);
        self.head = (self.head + 1) % self.array.capacity();
        self.length += 1;

        if self.length == self.array.capacity() {
            // self.extend();
        }
    }

    pub fn pop(&mut self) -> Option<u32> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;
        let ret = self.array[self.tail].unwrap();
        self.tail = (self.tail + 1) % self.array.capacity();

        // advance self.tail until we encounter Some value or self.tail == self.head
        while let None = self.array[self.tail] {
            if self.tail == self.head {
                break;
            }
            self.tail = (self.tail + 1) % self.array.capacity();
        }

        Some(ret)
    }

    // set entry at index i to None, return it's value
    pub fn remove(&mut self, i: usize) -> Option<u32> {
        let val = self.array[i];
        self.array[i] = None;
        self.length -= 1;
        val
    }

    // calculate the "size" of the contagious-wraparound array region [tail:head]
    fn wrapped_size(&self, head: usize, tail: usize, cap: usize) -> usize {
        if head < tail {
            // head wrapped around
            return head + cap - tail;
        }
        head - tail
    }

    // pack() DOES NOT reduce capacity of the underlying Vector
    fn pack(&mut self) {
        let mut write = self.tail;
        let mut read = write;

        while self.wrapped_size(self.tail, write, self.array.capacity()) < self.length {
            if let None = self.array[write] {
                // search from [write+1: end] until we find a Some() entry
                if read <= write {
                    read = (write + 1) % self.array.capacity();
                }

                while let None = self.array[read] {
                    read = (read + 1) % self.array.capacity();
                    // the read index can only move `i == array_capacity` indexes at most
                    if self.wrapped_size(read, self.tail, self.array.capacity())
                        >= self.array.capacity()
                    {
                        return;
                    }
                    // TODO some bound checking here
                }

                self.array.swap(read, write);
            }
            write = (write + 1) % self.array.capacity();
        }
    }
}
