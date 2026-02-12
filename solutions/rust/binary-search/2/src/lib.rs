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
        let value = &array[middle];

        if *value == key {
            return Some(middle);
        }

        if *value < key {
            start = middle + 1;
        } else {
            end = middle;
        }
    }
    None
}
