use std::cmp::Ordering;
use std::fmt::Debug;

pub fn find<R, T>(array: R, key: T) -> Option<usize>
where
    R: AsRef<[T]>,
    T: Ord + Debug,
{
    let array = array.as_ref();

    let mut start = 0;
    let mut end = array.len();

    while start < end {
        let middle = start + (end - start) / 2;

        match key.cmp(array.get(middle)?) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => start = middle + 1,
            Ordering::Less => end = middle,
        }
    }
    None
}
