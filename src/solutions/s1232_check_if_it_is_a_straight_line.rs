pub struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() == 2 {
            return true;
        };
        let slope = find_slope(&coordinates[0], &coordinates[1]);
        for coordinate in &coordinates[2..] {
            if find_slope(&coordinates[0], coordinate) != slope {
                return false;
            }
        }
        true
    }
}

/// Find the slope of the line by joining points `a` and `b`
fn find_slope(a: &[i32], b: &[i32]) -> f32 {
    let y_diff = b[1] - a[1];
    let x_diff = b[0] - a[0];
    let res = y_diff as f32 / x_diff as f32;
    if res == std::f32::NEG_INFINITY {
        std::f32::INFINITY
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case( vec![vec![0,0],vec![0,1],vec![0,-1]] => true ; "vertical")]
    #[test_case( vec![vec![0,1],vec![2,1],vec![-5,1]] => true ; "horizontal")]
    #[test_case(vec![vec![2,4],vec![3,6],vec![4,8]] => true ; "positive slope")]
    #[test_case(vec![vec![2,4],vec![1,6],vec![0,8]] => true ; "negative slope")]
    #[test_case( vec![vec![1,1],vec![2,2],vec![2,0]] => false ; "not straight")]
    fn test_1232(x: Vec<Vec<i32>>) -> bool {
        Solution::check_straight_line(x)
    }
}
