// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `da_control`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `fedora`, CPU: `13th Gen Intel(R) Core(TM) i7-13700K`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/data-avail
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=da_control
// --extrinsic=*
// --heap-pages=4096
// --header=./HEADER-APACHE2
// --log=warn
// --output
// ./output/da_control.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `da_control`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> da_control::WeightInfo for WeightInfo<T> {
	/// Storage: `DataAvailability::AppKeys` (r:1 w:1)
	/// Proof: `DataAvailability::AppKeys` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `DataAvailability::NextAppId` (r:1 w:1)
	/// Proof: `DataAvailability::NextAppId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn create_application_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117`
		//  Estimated: `3583`
		// Minimum execution time: 10_086_000 picoseconds.
		Weight::from_parts(10_695_000, 0)
			.saturating_add(Weight::from_parts(0, 3583))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `System::DynamicBlockLength` (r:1 w:1)
	/// Proof: `System::DynamicBlockLength` (`max_values`: Some(1), `max_size`: Some(24), added: 519, mode: `MaxEncodedLen`)
	fn submit_block_length_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65`
		//  Estimated: `1509`
		// Minimum execution time: 8_047_000 picoseconds.
		Weight::from_parts(8_482_000, 0)
			.saturating_add(Weight::from_parts(0, 1509))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// The range of component `i` is `[1, 524288]`.
	fn submit_data(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_092_000 picoseconds.
		Weight::from_parts(6_027_717, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(791, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[0, 524288]`.
	fn data_root(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_203_000 picoseconds.
		Weight::from_parts(4_556_627, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(1_836, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[0, 2097152]`.
	fn data_root_batch(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_918_000 picoseconds.
		Weight::from_parts(9_247_936, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(1_831, 0).saturating_mul(i.into()))
	}
}
