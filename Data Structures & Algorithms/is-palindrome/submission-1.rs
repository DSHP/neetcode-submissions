impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.to_lowercase().into_bytes();
        let mut left = 0;
        let mut right = s.len();

        while left < right {
            right -= 1;
            
            while left < right && !s[left].is_ascii_alphanumeric() {
                left += 1;
            }

            while left < right && !s[right].is_ascii_alphanumeric() {
                right -= 1;
            }

            if left < right && s[left] != s[right] {
                return false;
            }

            left += 1;
        }

        true
    }
}
