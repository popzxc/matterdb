// Copyright 2020 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(clippy::ptr_arg)] // Usage of `&Vec<_>` is needed for type inference

// cspell:ignore oneof

//! Property testing for list index and proof list index as a rust collection.

use modifier::Modifier;
use proptest::{
    collection::vec, num, prop_assert, prop_oneof, proptest, strategy, strategy::Strategy,
    test_runner::TestCaseResult,
};

use std::rc::Rc;

use matterdb::{access::AccessExt, BinaryValue, Fork, ListIndex, TemporaryDB};

mod common;

use crate::common::{compare_collections, AsForkAction, ForkAction, FromFork, ACTIONS_MAX_LEN};

#[derive(Debug, Clone)]
enum ListAction<V> {
    Push(V),
    Pop,
    Extend(Vec<V>),
    // Applied with argument modulo `collection.len()`.
    Truncate(u64),
    // Applied to index modulo `collection.len()`.
    Set(u64, V),
    Clear,
    MergeFork,
}

impl<V> AsForkAction for ListAction<V> {
    fn as_fork_action(&self) -> Option<ForkAction> {
        match self {
            ListAction::MergeFork => Some(ForkAction::Merge),
            _ => None,
        }
    }
}

impl<V> Modifier<Vec<V>> for ListAction<V> {
    fn modify(self, list: &mut Vec<V>) {
        match self {
            ListAction::Push(val) => {
                list.push(val);
            }
            ListAction::Pop => {
                list.pop();
            }
            ListAction::Extend(vec) => {
                list.extend(vec);
            }
            ListAction::Truncate(size) => {
                let len = list.len();
                if len > 0 {
                    list.truncate(size as usize % len);
                }
            }
            ListAction::Set(idx, val) => {
                let len = list.len();
                if len > 0 {
                    list[idx as usize % len] = val;
                }
            }
            ListAction::Clear => {
                list.clear();
            }
            _ => unreachable!(),
        }
    }
}

impl<V: BinaryValue> Modifier<ListIndex<Rc<Fork>, V>> for ListAction<V> {
    fn modify(self, list: &mut ListIndex<Rc<Fork>, V>) {
        match self {
            ListAction::Push(val) => {
                list.push(val);
            }
            ListAction::Pop => {
                list.pop();
            }
            ListAction::Extend(vec) => {
                list.extend(vec);
            }
            ListAction::Truncate(size) => {
                let len = list.len();
                if len > 0 {
                    list.truncate(size % len);
                }
            }
            ListAction::Set(idx, val) => {
                let len = list.len();
                if len > 0 {
                    list.set(idx % len, val);
                }
            }
            ListAction::Clear => {
                list.clear();
            }
            _ => unreachable!(),
        }
    }
}

impl<V: BinaryValue> FromFork for ListIndex<Rc<Fork>, V> {
    fn from_fork(fork: Rc<Fork>) -> Self {
        fork.get_list("test")
    }

    fn clear(&mut self) {
        self.clear();
    }
}

fn generate_action() -> impl Strategy<Value = ListAction<i32>> {
    prop_oneof![
        num::i32::ANY.prop_map(ListAction::Push),
        strategy::Just(ListAction::Pop),
        vec(num::i32::ANY, 1..5).prop_map(ListAction::Extend),
        num::u64::ANY.prop_map(ListAction::Truncate),
        (num::u64::ANY, num::i32::ANY).prop_map(|(i, v)| ListAction::Set(i, v)),
        strategy::Just(ListAction::Clear),
        strategy::Just(ListAction::MergeFork),
    ]
}

fn compare_list(list: &ListIndex<Rc<Fork>, i32>, ref_list: &Vec<i32>) -> TestCaseResult {
    prop_assert!(ref_list.iter().copied().eq(list));
    Ok(())
}

#[test]
fn compare_list_to_vec() {
    let db = TemporaryDB::new();
    proptest!(|(ref actions in vec(generate_action(), 1..ACTIONS_MAX_LEN))| {
        compare_collections(&db, actions, compare_list)?;
    });
}
