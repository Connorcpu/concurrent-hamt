#![feature(thread_local, box_syntax, core_intrinsics, borrow_state)]

//! A Concurrent, persistent, wait-free, non-blocking, hash map array trie.

extern crate rand;

pub mod hamt;
pub mod hp;
mod bits;
