use crate::internal_prelude::*;

/// Stores a buffer Vec<T>, and an index into that buffer. This acts like an Iterator, except the
/// current index can be moved backwards or to an arbitrary location in the buffer.
///
/// Note that any time a method returns an error, it does not modify the cursor. For example, if
/// moving the Cursor's index would cause an OOB error, the index will be unmodified.
pub struct Cursor<T> {
    buffer: Vec<T>,
    index: usize,
}

impl<T> Cursor<T> {
    /// Attempts to make a new Cursor from a buffer. Returns None if the buffer is empty. This
    /// safeguard is here because current_value can panic if the buffer is empty.
    pub fn new(buffer: Vec<T>) -> Option<Self> {
        if !buffer.is_empty() {
            Some(Self { buffer, index: 0 })
        } else {
            None
        }
    }
    /// Gets the index the cursor is currently at in the buffer.
    pub fn pos(&self) -> usize {
        self.index
    }

    /// Attempts to move to a specific position in the buffer. Returns Err if the new location would
    /// be out of bounds.
    pub fn move_to(&mut self, pos: usize) -> Result<(), ()> {
        if pos < self.buffer.len() {
            self.index = pos;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Attempts to move the index a certain amount forward. Returns Err if adding amount would cause
    /// overflow or the new location would be out of bounds.
    pub fn advance_by(&mut self, amount: usize) -> Result<(), ()> {
        let index = self.index.checked_add(amount);
        if let Some(index) = index {
            if index < self.buffer.len() {
                self.index = index;
                return Ok(());
            }
        }

        Err(())
    }

    /// Attempts to move the index a certain amount backwards. Returns Err if the new location would
    /// be less than 0.
    pub fn retreat_by(&mut self, amount: usize) -> Result<(), ()> {
        let index = self.index.checked_sub(amount);
        if let Some(index) = index {
            self.index = index;
            return Ok(());
        }

        Err(())
    }

    /// Moves the index forwards or backwards by offset, depending on if offset is positive or negative.
    /// Returns Err if adding the offset would cause overflow, underflow, or go out of bounds.
    pub fn offset_by(&mut self, offset: isize) -> Result<(), ()> {
        if offset >= 0 {
            self.advance_by(offset as usize)
        } else {
            self.retreat_by(offset.unsigned_abs())
        }
    }

    /// Attempts to move the index forwards by 1, then get the new value at that index. Returns None
    /// if index + 1 would cause overflow or is out of bounds.
    pub fn next_val(&mut self) -> Option<&T> {
        self.offset_by(1).ok()?;

        Some(&self.buffer[self.index])
    }

    /// Gets the current value at the cursor's index.
    pub fn current_value(&self) -> &T {
        &self.buffer[self.index]
    }

    /// Attempts to move the index backwards by 1, then get the new value at that index. Returns
    /// None if the new index would be less than 0.
    pub fn previous_value(&mut self) -> Option<&T> {
        self.offset_by(-1).ok()?;

        Some(&self.buffer[self.index])
    }

    /// Attempts to get the next N values from the buffer. Returns None if index + N is out of bounds.
    /// This method gets the **next** N values from the buffer; the resulting array does not include
    /// the current value.
    ///
    /// For a non-const version of this function that returns a slice instead of an array, see
    /// next_values.
    pub fn next_n_values<const N: usize>(&mut self) -> Option<&[T; N]> {
        self.advance_by(1).ok()?;
        let start = self.index;
        // If this advancement fails, we should undo the previous one
        if self.advance_by(N - 1).is_err() {
            self.index -= 1
        }
        let end = self.index;

        Some(self.buffer[start..=end].try_into().unwrap())
    }

    /// Attempts to get the next count values from the buffer. Returns None if index + count is out
    /// of bounds. This method gets the **next** count values from the buffer; the resulting slice
    /// does not include the current value.
    ///
    /// For a const version of this function that returns an array instead of a slice, see
    /// next_n_values.
    pub fn next_values(&mut self, count: usize) -> Option<&[T]> {
        self.advance_by(1).ok()?;
        let start = self.index;
        // If this advancement fails, we should undo the previous one
        if self.advance_by(count - 1).is_err() {
            self.index -= 1
        }
        let end = self.index;

        Some(&self.buffer[start..=end])
    }

    /// Attempts to get the value at index + 1, without modifying index. Returns None if adding 1
    /// would cause overflow or the new index is out of bounds.
    pub fn peek_next_value(&self) -> Option<&T> {
        self.buffer.get(self.index.checked_add(1)?)
    }

    /// Attempts to get the value at index - 1, without modifying index. Returns None if subtracting
    /// 1 would cause underflow.
    pub fn peek_previous_value(&self) -> Option<&T> {
        self.buffer.get(self.index.checked_sub(1)?)
    }

    /// The number of values in the cursor's buffer that are past the cursor's index.
    pub fn remaining_values(&self) -> usize {
        self.buffer.len() - self.index - 1
    }
}

pub struct CursorIter<'a, T> {
    cursor: &'a Cursor<T>,
}

impl<'a, T> core::iter::Iterator for CursorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.get(0)
    }
}

impl<T> core::ops::Deref for Cursor<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
