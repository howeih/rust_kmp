    extern crate kmp;
use kmp::KMP;

fn main(){
    let pattern = "abcabca";
    let kmp = KMP::new(pattern);
    debug_assert_eq!(3, kmp.index_of_any("abxabcabcaby"));
    debug_assert_eq!(-1, kmp.index_of_any("abxabdabcaby"));
    }