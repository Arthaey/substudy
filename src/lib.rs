//! Tools for studying foreign languages using subtitles.  All APIs are
//! currently experimental or unstable, but if you'd like me to stabilize
//! things, please get in touch.
//!
//! For further details about how to use substudy, see [the main GitHub
//! page](http://github.com/emk/substudy).

#![warn(missing_docs)]

extern crate cld2;
extern crate csv;
#[cfg(test)] extern crate difference;
extern crate encoding;
#[macro_use] extern crate error_chain;
extern crate handlebars;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate num;
extern crate regex;
extern crate rustc_serialize;
extern crate uchardet;

pub mod errors;
pub mod contexts;
pub mod decode;
mod grammar;
pub mod lang;
pub mod srt;
pub mod clean;
pub mod merge;
pub mod time;
pub mod align;
pub mod video;
pub mod export;
