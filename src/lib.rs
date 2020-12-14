//! # Advent of Code 2020
//!
//! After saving Christmas five years in a row, you've decided to take a
//! vacation at a nice resort on a tropical island. Surely, Christmas will go
//! on without you.
//!
//! The tropical island has its own currency and is entirely cash-only. The
//! gold coins used there have a little picture of a starfish; the locals just
//! call them stars. None of the currency exchanges seem to have heard of them,
//! but somehow, you'll need to find fifty of these coins by the time you
//! arrive so you can pay the deposit on your room.
//!
//! To save your vacation, you need to get all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on
//! each day in the Advent calendar; the second puzzle is unlocked when you
//! complete the first. Each puzzle grants one star. Good luck!
//!
//! [Advent of Code 2020](https://adventofcode.com/2020)

#![deny(unsafe_code)]
#![warn(
    bare_trait_objects,
    broken_intra_doc_links,
    missing_copy_implementations,
    missing_debug_implementations,
    private_doc_tests,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
//#![warn(missing_docs)] //TODO uncomment eventually

#[macro_use]
extern crate aoc_runner_derive;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

aoc_lib! { year = 2020 }
