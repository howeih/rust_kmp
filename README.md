KMP(Knuth–Morris–Pratt algorithm) implement in Rust
=================

In computer science, the Knuth–Morris–Pratt string searching algorithm (or KMP algorithm) searches for occurrences of a "word" W within a main "text string" S by employing the observation that when a mismatch occurs, the word itself embodies sufficient information to determine where the next match could begin, thus bypassing re-examination of previously matched characters.

### Installation:
Cargo.toml:
```
    kmp = { git = "https://github.com/howeih/rust_kmp.git", branch = "master" }
```

### Usage :
```
    extern crate kmp;
    use kmp::KMP;


    let pattern = "abcabca";
    let kmp = KMP::new(pattern);
    debug_assert_eq!(3, kmp.index_of_any("abxabcabcaby"));
    debug_assert_eq!(-1, kmp.index_of_any("abxabdabcaby"));

```