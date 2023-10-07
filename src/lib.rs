#![no_std]
#![feature(allocator_api)]

extern crate alloc;

#[cfg(feature = "tokens")]
mod tokenizer;

use {
    alloc::vec::Vec,
    core::{
        default::Default,
        iter::{IntoIterator, Iterator},
        marker::PhantomData,
        slice::{Iter, IterMut},
    },
};

pub mod prelude {
    #[cfg(feature = "tokens")]
    pub use super::tokenizer::*;

    pub use super::*;
}

pub struct Cursor<T> {
    input: Vec<T>,
    index: usize,
}
impl<T> Cursor<T> {
    pub fn pos(&self) -> usize {
        self.index
    }
    pub fn move_to(&mut self, pos: usize) -> Result<(), ()> {
        if pos < self.input.len() {
            self.index = pos;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn advance_by(&mut self, amount: usize) -> Result<(), ()> {
        let index = self.index.checked_add(amount);
        if let Some(index) = index {
            if index < self.input.len() {
                self.index = index;
                return Ok(());
            }
        }

        Err(())
    }
    pub fn retreat_by(&mut self, amount: usize) -> Result<(), ()> {
        let index = self.index.checked_sub(amount);
        if let Some(index) = index {
            self.index = index;
            return Ok(());
        }

        Err(())
    }
    pub fn offset_by(&mut self, offset: isize) -> Result<(), ()> {
        if offset >= 0 {
            self.advance_by(offset as usize)
        } else {
            self.retreat_by(offset.unsigned_abs())
        }
    }

    pub fn next_val(&mut self) -> Option<&T> {
        self.offset_by(1).ok()?;

        Some(&self.input[self.index])
    }
    pub fn next_n_values<const N: usize>(&mut self) -> Option<&[T; N]> {
        self.advance_by(1).ok()?;
        let start = self.index;
        // If this advancement fails, we should undo the previous one
        if self.advance_by(N - 1).is_err() {
            self.index -= 1
        }
        let end = self.index;

        Some(self.input[start..=end].try_into().unwrap())
    }
    pub fn next_values(&mut self, bytes: usize) -> Option<&[T]> {
        self.advance_by(1).ok()?;
        let start = self.index;
        // If this advancement fails, we should undo the previous one
        if self.advance_by(bytes - 1).is_err() {
            self.index -= 1
        }
        let end = self.index;

        Some(&self.input[start..=end])
    }
    pub fn current_value(&self) -> &T {
        &self.input[self.index]
    }
    pub fn previous_value(&mut self) -> Option<&T> {
        self.offset_by(-1).ok()?;

        Some(&self.input[self.index])
    }

    pub fn peek_next_value(&self) -> Option<&T> {
        self.input.get(self.index.checked_add(1)?)
    }
    pub fn peek_previous_value(&self) -> Option<&T> {
        self.input.get(self.index.checked_sub(1)?)
    }

    pub fn peek_range(&self, min: usize, max: usize) -> Option<&[T]> {
        if max < self.input.len() {
            Some(&self.input[min..max])
        } else {
            None
        }
    }

    /// How many values are left in the input, including the current value.
    pub fn remaining_values(&self) -> usize {
        self.input.len() - self.index
    }
}

pub trait Rule {
    type Input;

    fn matches(input: &Self::Input) -> bool;
    fn deserialize(input: &mut Self::Input) -> Self;
}
pub trait SerializableRule: Rule {
    fn serialize(self) -> Self::Input;
}

pub trait MainRule {
    type Input;

    fn matches(input: &Self::Input) -> bool;
    fn deserialize(input: Self::Input) -> Self;
}

#[derive(Default)]
pub struct Parser<MainRuleTy: MainRule<Input = Cursor<InputType>>, InputType> {
    _main_rule: PhantomData<MainRuleTy>,
}
impl<InputType, MainRuleTy: MainRule<Input = Cursor<InputType>>> Parser<MainRuleTy, InputType> {
    pub fn parse(input: Vec<InputType>) -> Option<MainRuleTy> {
        let input = Cursor { input, index: 0 };

        if MainRuleTy::matches(&input) {
            Some(MainRuleTy::deserialize(input))
        } else {
            None
        }
    }
}
