use std::cmp::{Ordering};
use std::collections::{BinaryHeap};

#[derive(Eq, PartialEq, PartialOrd, Debug)]
pub struct Length(pub String);

impl Ord for Length {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.len().cmp(&other.0.len())
    }
}

pub struct HeapDrain<'a, T> where T: 'a + Ord {
    heap: &'a mut BinaryHeap<T>,
}

pub trait HeapPopDrain<T> where T: Ord {
    fn pop_drain(&mut self) -> HeapDrain<T>;
}

impl<T> HeapPopDrain<T> for BinaryHeap<T> where T: Ord {
    fn pop_drain(&mut self) -> HeapDrain<T> {
        HeapDrain{
            heap: self,
        }
    }
}

impl<'a, T> Iterator for HeapDrain<'a, T> where T: 'a + Ord {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_length() {
        use super::{Length};
        use std::cmp::{Ordering};

        let short = Length("short".to_string());
        let longer = Length("longer".to_string());
        let equal = Length("equal".to_string());

        assert_eq!(short.cmp(&longer), Ordering::Less);
        assert_eq!(longer.cmp(&short), Ordering::Greater);
        assert_eq!(short.cmp(&equal), Ordering::Equal);
    }

    #[test]
    fn test_heapdrain() {
        use super::{HeapPopDrain};
        use std::collections::{BinaryHeap};

        let mut heap = BinaryHeap::new();
        heap.push(3);
        heap.push(9);
        heap.push(6);

        let mut drain = heap.pop_drain();
        assert_eq!(drain.next(), Some(9));
        assert_eq!(drain.next(), Some(6));
        assert_eq!(drain.next(), Some(3));
        assert_eq!(drain.next(), None);
    }
}
