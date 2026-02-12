pub fn find<T: Ord>(array: &[T], key: T) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut start = 0;
    let mut end = array.len() - 1;

    while start <= end {
        let middle = start + (end - start) / 2;
        let value = &array[middle];

        if *value == key {
            return Some(middle);
        }
        if *value < key {
            start = middle + 1;
        } else if middle == 0 {
            break;
        } else {
            end = middle - 1;
        }
    }
    None
}
