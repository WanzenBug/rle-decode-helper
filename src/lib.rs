use std::{
    ptr,
    ops,
    cmp,
};

pub fn rle_decode<T>(
    buffer: &mut Vec<T>,
    mut repeating_fragment_len: usize,
    mut items_to_fill: usize,
) where T: Copy {
    assert_ne!(repeating_fragment_len, 0, "attempt to repeat fragment of size 0");
    
    let copy_fragment_start = buffer.len()
        .checked_sub(repeating_fragment_len)
        .expect("attempt to repeat fragment larger than buffer size");

    // Reserve space for copies
    buffer.reserve(items_to_fill);
 
    
    while items_to_fill > 0 {
        let fill_size = cmp::min(repeating_fragment_len, items_to_fill);
        append_from_within(
            buffer,
            copy_fragment_start..(copy_fragment_start + fill_size)
        );
        items_to_fill -= fill_size;
        repeating_fragment_len *= 2;
    }
}

fn append_from_within<T, R: ops::RangeBounds<usize>>(seif: &mut Vec<T>, src: R) where T: Copy, {
    let src_start = match src.start_bound() {
        ops::Bound::Included(&n) => n,
        ops::Bound::Excluded(&n) => n
            .checked_add(1)
            .unwrap_or_else(|| vec_index_overflow_fail()),
        ops::Bound::Unbounded => 0,
    };
    let src_end = match src.end_bound() {
        ops::Bound::Included(&n) => n
            .checked_add(1)
            .unwrap_or_else(|| vec_index_overflow_fail()),
        ops::Bound::Excluded(&n) => n,
        ops::Bound::Unbounded => seif.len(),
    };
    assert!(src_start <= src_end, "src end is before src start");
    assert!(src_end <= seif.len(), "src is out of bounds");
    let count = src_end - src_start;
    seif.reserve(count);
    let vec_len = seif.len();
    unsafe {
        // This is safe because reserve() above succeeded,
        // so `seif.len() + count` did not overflow usize
        ptr::copy_nonoverlapping(
            seif.get_unchecked(src_start),
            seif.get_unchecked_mut(vec_len),
            count,
        );
        seif.set_len(vec_len + count);
    }
}

#[inline(never)]
#[cold]
fn vec_index_overflow_fail() -> ! {
    panic!("attempted to index vec up to maximum usize");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut buf = vec![1, 2, 3, 4, 5];
        rle_decode(&mut buf, 3, 10);
        assert_eq!(buf, &[1, 2, 3, 4, 5, 3, 4, 5, 3, 4, 5, 3, 4, 5, 3]);
    }
    
    #[test]
    fn test_zero_repeat() {
        let mut buf = vec![1, 2, 3, 4, 5];
        rle_decode(&mut buf, 3, 0);
        assert_eq!(buf, &[1, 2, 3, 4, 5]);
    }
    
    #[test]
    #[should_panic]
    fn test_zero_fragment() {
        let mut buf = vec![1, 2, 3, 4, 5];
        rle_decode(&mut buf, 0, 10);
    }
    
    #[test]
    #[should_panic]
    fn test_zero_fragment_and_repeat() {
        let mut buf = vec![1, 2, 3, 4, 5];
        rle_decode(&mut buf, 0, 0);
    }
    
    #[test]
    #[should_panic]
    fn test_overflow_fragment() {
        let mut buf = vec![1, 2, 3, 4, 5];
        rle_decode(&mut buf, 10, 10);
    }
    
    #[test]
    #[should_panic]
    fn test_overflow_buf_size() {
        let mut buf = vec![1, 2, 3, 4, 5];
        rle_decode(&mut buf, 4, usize::max_value());
    }
}
