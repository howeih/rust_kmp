pub struct KMP {
    pattern: Vec<char>,
    failure_function: Vec<usize>,
    pattern_length: usize
}

impl KMP {
    pub fn new(pattern: &str) -> KMP {
        let pattern: Vec<char> = pattern.chars().collect();
        let pattern_length = pattern.len();
        KMP {
            failure_function: KMP::find_failure_function(&pattern),
            pattern: pattern,
            pattern_length: pattern_length
        }
    }

    fn find_failure_function(pattern: &Vec<char>) -> Vec<usize>{
        let mut i = 1;
        let mut j = 0;
        let pattern_length = pattern.len();
        let end_i = pattern_length - 1;
        let mut failure_function = vec![0usize; pattern_length];
        while i <= end_i {
            if pattern[i] == pattern[j] {
                failure_function[i] = j + 1;
                i = i + 1;
                j = j + 1;
            } else {
                if j == 0 {
                    failure_function[i] = 0;
                    i = i + 1;
                } else {
                    j = failure_function[j - 1];
                }
            }
        }
        failure_function
    }

    pub fn index_of_any(&self, target: &str) -> i32 {
        let target: Vec<char> = target.chars().collect();
        let mut t_i: usize = 0;
        let mut p_i: usize = 0;
        let target_len = target.len();
        let mut result_idx = 0i32;
        let pattern_len = self.pattern_length;
        while (t_i <= target_len - 1) && (p_i <= pattern_len - 1) {
            if target[t_i] == self.pattern[p_i] {
                if result_idx == 0 {
                    result_idx = t_i as i32;
                }
                t_i = t_i + 1;
                p_i = p_i + 1;
                if p_i >= pattern_len{
                    return result_idx
                }
            } else {
                if p_i == 0 {
                    p_i = 0;
                } else {
                    p_i = self.failure_function[p_i - 1];
                }
                t_i = t_i + 1;
                result_idx = 0;
            }
        }
        -1
    }
}
