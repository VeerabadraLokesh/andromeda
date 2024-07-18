impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let s = n*(n+1)/2;
        let x = (f32::sqrt(s as f32)) as i32;
        if x * x == s {x} else {-1}
    }
}

/* */

