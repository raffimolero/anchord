use std::num::NonZeroI8;

type Key = NonZeroI8;

fn combo_to_num(a: Key, b: Option<Key>) -> i32 {
    let Some(b) = b else {
        return a.get() as i32;
    }
}

fn main() {}
