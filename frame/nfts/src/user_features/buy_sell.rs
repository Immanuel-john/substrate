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

use crate::*;
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement::KeepAlive},
};

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	pub fn do_pay_tips(sender: T::AccountId, tips: Vec<ItemTip<T, I>>) -> DispatchResult {
		for tip in tips {
			T::Currency::transfer(&sender, &tip.2, tip.3, KeepAlive)?;
			Self::deposit_event(Event::TipSent {
				collection: tip.0,
				item: tip.1,
				sender: sender.clone(),
				receiver: tip.2,
				amount: tip.3,
			});
		}
		Ok(())
	}
}
