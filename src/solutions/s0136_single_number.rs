pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |a, b| a ^ b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case( vec![1,2,3,1,2] => 3 ; "middle")]
    #[test_case( vec![1,2,1,2,3] => 3 ; "last")]
    #[test_case(vec![1,2,3,2,3] => 1 ; "first")]
    fn test_0136(x: Vec<i32>) -> i32 {
        Solution::single_number(x)
    }
}
