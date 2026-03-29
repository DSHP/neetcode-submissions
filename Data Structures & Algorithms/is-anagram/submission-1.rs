impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut alpha: [i32; 26] = [0; 26];

        for c in s.bytes() {
            let i = (c - b'a') as usize;
            alpha[i] += 1;
        }

        for c in t.bytes() {
            let mut i = (c - b'a') as usize;
            alpha[i] -= 1;
            if alpha[i] < 0 {
                return false;
            }
        }

        true 
    }
}
