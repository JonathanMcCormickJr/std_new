#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

use std::panic::{RefUnwindSafe, UnwindSafe};

use serde::{Deserialize, Serialize};
use static_assertions::assert_impl_all;

// Implement common traits which may be usesful. 
// `Copy` is commented out as it may not always be appropriate.
#[derive(Clone, /*Copy,*/ Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ExampleStruct {
    pub value: i32,
}

// Ensure ExampleStruct implements common auto traits. 
// If any of these traits are not implemented, a compile-time error will occur, and 
// thus a major version bump may be required.
assert_impl_all!(ExampleStruct: RefUnwindSafe, Send, Sized, Sync, Unpin, UnwindSafe);