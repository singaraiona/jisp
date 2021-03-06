#![feature(iterator_try_fold)]
#![feature(box_patterns)]
#![feature(core_intrinsics)]
#![feature(plugin)]
#![plugin(dynasm)]

#[macro_use]
extern crate nom;
extern crate fnv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate dynasmrt;

#[macro_use]
pub mod eval;
#[macro_use]
pub mod parser;
