use std::ops::Range;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/// 當兩個範圍重疊時回傳true
///
///     assert_eq!(ranges::overlap(0..7,3..10), true);
///     assert_eq!(ranges::overlap(1..5,101..105), false);
///
/// 如果其中一個範圍是空的，則回傳false
///
///     assert_eq!(ranges::overlap(0..0,0..10), false);
///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}
