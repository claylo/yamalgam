//! Streaming multi-document iterator.

use serde::Deserialize;
use yamalgam_parser::Event;

use crate::de::Deserializer;
use crate::Error;

/// Iterator over YAML documents in a stream.
///
/// Created by [`Deserializer::documents`]. Each call to `next()` deserializes
/// one YAML document into `T`. Iteration stops at `StreamEnd` or on the
/// first error.
///
/// Unlike [`from_str`](crate::from_str), this does **not** reject
/// multi-document inputs and does **not** call `check_end()`.
pub struct Documents<'input, T> {
    de: Deserializer<'input>,
    done: bool,
    _marker: std::marker::PhantomData<T>,
}

impl<'input, T> Documents<'input, T> {
    pub(crate) const fn new(de: Deserializer<'input>) -> Self {
        Self {
            de,
            done: false,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'input, T> Iterator for Documents<'input, T>
where
    T: Deserialize<'input>,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Skip DocumentEnd events between documents and detect StreamEnd.
        loop {
            let event = match self.de.peek_raw_event() {
                Ok(ev) => ev,
                Err(e) => {
                    self.done = true;
                    return Some(Err(e));
                }
            };

            match event {
                Event::StreamEnd => {
                    self.done = true;
                    return None;
                }
                Event::DocumentEnd { .. } => {
                    // Consume the DocumentEnd and loop to check for next doc.
                    if let Err(e) = self.de.next_raw_event() {
                        self.done = true;
                        return Some(Err(e));
                    }
                    continue;
                }
                _ => break,
            }
        }

        // Deserialize one T from the current document's content.
        match T::deserialize(&mut self.de) {
            Ok(value) => Some(Ok(value)),
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }
}
