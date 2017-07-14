// Copyright 2014 Steve Klabnik, Valerii Hiora, Oliver Mader
// Copyright 2015 Carl Lerche, Oliver Mader, Alex Crichton, Graham Dennis,
//                Tamir Duberstein, Robin Gloster
// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::any::TypeId;
use std::fmt::{self, Display, Formatter};

use core::*;

pub struct TypeOf(TypeId);

impl Display for TypeOf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "type_of({:?})", self.0)
    }
}

impl<T: 'static> Matcher<T> for TypeOf {
    fn matches(&self, _: T) -> MatchResult {
        let type_id = TypeId::of::<T>();

        if self.0 == type_id {
            success()
        } else {
            Err(format!("type_of({:?})", type_id))
        }
    }
}

pub fn type_of<T: 'static>() -> TypeOf {
    TypeOf(TypeId::of::<T>())
}
