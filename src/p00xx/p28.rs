//!
//! Implement strStr()
//!
//! https://leetcode.com/problems/implement-strstr/
//!
//! Implement [strStr()](http://www.cplusplus.com/reference/cstring/strstr/).
//!
//! Return the index of the first occurrence of needle in haystack, or **-1** if needle is not part of haystack.
//!
//! **Example 1:**
//! ```text
//! Input: haystack = "hello", needle = "ll"
//! Output: 2
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: haystack = "aaaaa", needle = "bba"
//! Output: -1
//! ```
//!
//! **Clarification:**
//!
//! What should we return when `needle` is an empty string?
//!
//! This is a great question to ask during an interview.
//!
//! For the purpose of this problem, we will return 0 when `needle` is an empty string.
//!
//! This is consistent to C's [strstr()](http://www.cplusplus.com/reference/cstring/strstr/) and Java's [indexOf()](https://docs.oracle.com/javase/7/docs/api/java/lang/String.html#indexOf(java.lang.String)).
//!
//!


#[derive(Debug, Clone)]
pub struct Input {
    pub haystack: String, 
    pub needle  : String,
}
pub type Output = i32;

pub trait Solution {
    fn str_str(&self, haystack: String, needle: String) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Rust build-in method.
pub struct Solution0;
impl Solution for Solution0 {

    fn str_str(&self, haystack: String, needle: String) -> i32 {

        haystack.find(&needle)
            .map(|first| first as i32)
            .unwrap_or(-1)
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Brute Force.
pub struct Solution1;
impl Solution for Solution1 {

    fn str_str(&self, haystack: String, needle: String) -> i32 {

        let haystack: Vec<char> = haystack.chars().collect();
        let needle  : Vec<char> = needle.chars().collect();

        if needle.is_empty() {
            return 0
        }

        for i in 0..haystack.len() {
            if haystack[i] == needle[0] {

                if i + needle.len() > haystack.len() {
                    return -1
                }

                let mut flag = true;
                for j in 0..needle.len() {
                    if haystack[i + j] != needle[j] {
                        flag = false;
                        break
                    }
                }

                if flag {
                    return i as i32
                }
            }
        }
        
        -1
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 2: KMP alogrithm.
pub struct Solution2;
impl Solution for Solution2 {

    fn str_str(&self, haystack: String, needle: String) -> i32 {
        kmp(&needle, &haystack)
    }
}

fn kmp(pattern: &str, test: &str) -> i32 {

    let gen_next = |pattern: &[u8]| -> Vec<i32> {
     
        let mut nexts = vec![-1; pattern.len() + 1];
        nexts[0] = -1;

        let mut i: usize = 0;
        let mut j: i32 = -1;

        while i < pattern.len() {

            if j == -1 || pattern[i] == pattern[j as usize] {
                i += 1;
                j += 1;

                nexts[i] = j;
            } else {
                j = nexts[j as usize];
            }
        }

        nexts
    };

    if pattern.is_empty() { return 0 }
    let pattern : &[u8] = pattern.as_bytes();
    let test    : &[u8] = test.as_bytes();

    let nexts = gen_next(&pattern);

    let mut i: usize = 0;
    let mut j: i32 = 0;
    
    while i < test.len() {

        if j == -1 || test[i] == pattern[j as usize] {
            i += 1;
            j += 1;
            
            if (j as usize) == pattern.len() {
                return i as i32 - j
            }
        } else {
            j = nexts[j as usize];
        }
    }

    -1
}
// -----------------------------------------------------------------------------

