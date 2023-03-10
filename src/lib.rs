// Copyright 2013-2014 The Algebra Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Traits for algebra.

/*
#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_results)]
#![deny(missing_docs)]
*/
#![cfg_attr(not(feature = "std"), no_std)]

#[allow(unused_imports)]
#[macro_use]
extern crate approx;

#[cfg(not(feature = "std"))]
extern crate core as std;

pub mod ops;

#[cfg(feature = "structures")]
pub mod operator;
#[cfg(feature = "structures")]
pub use operator::*;

#[cfg(feature = "structures")]
pub mod properties;
#[cfg(feature = "structures")]
pub use properties::*;

#[cfg(feature = "structures")]
pub mod structures;
#[cfg(feature = "structures")]
pub use structures::*;

#[doc(hidden)]
#[cfg(feature = "structures")]
pub mod wrapper;

