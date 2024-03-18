//!
//! Drive for the CC2420 IEEE 802.15.4 Compatible Radio Module
//! 

#![no_std]

extern crate alloc;

mod register;

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
