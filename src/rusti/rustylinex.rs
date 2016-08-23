// Copyright 2014-2016 Rusti Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustyline;

use std::sync::Mutex;

use self::rustyline::Editor;

lazy_static! {
    // TODO(akavel): use completion from readline.rs
    static ref RL: Mutex<Editor<()>> = Mutex::new(Editor::<()>::new());
}

/// Reads a line from the input stream. The trailing newline is truncated.
/// Returns `None` if end-of-file is signaled.
pub fn read_line(prompt: &str) -> Option<String> {
    let line = RL.lock().unwrap().readline(prompt);
    match line {
        Ok(text) => Some(text),
        Err(_)   => None,
    }
}

/// Pushes a single line into `readline` history.
pub fn push_history(line: &str) {
}

