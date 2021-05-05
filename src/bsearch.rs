use std::cmp::Ordering::{self, Equal, Greater, Less};

pub fn binary_search_by(
    mut size: usize,
    mut f: impl FnMut(usize) -> Ordering,
) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = size;
    while left < right {
        let mid = left + size / 2;
        match f(mid) {
            Less => {
                left = mid + 1;
            }
            Equal => {
                return Ok(mid);
            }
            Greater => {
                right = mid;
            }
        }
        size = right - left;
    }
    Err(left)
}
