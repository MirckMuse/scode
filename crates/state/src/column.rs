use crate::char::{find_cluster_break};
use crate::UnicodeCodePoint;

pub fn count_column(string: String, tab_size: i32, to: Option<usize>) -> usize {
    let to = to.unwrap_or(string.len());
    let mut n: usize = 0;
    let mut i = 0;

    while i < to {
        if string.char_code_at(i) == 0 {
            n += tab_size - (n % tab_size);
            i + 1;
        } else {
            n += 1;
            i = find_cluster_break(&string, i, None, None);
        }
    }

    return n;
}

pub fn find_column(string: String, col: usize, tab_size: usize, strict: Option<bool>) -> Option<usize> {
    let mut i = 0;
    let mut n = 0;

    loop {
        if n >= col {
            return Some(i);
        }

        if i == string.len() {
            break;
        }

        n += if string.char_code_at(i) == 9 { tab_size - (n % tab_size) } else { 1 };
        i = find_cluster_break(&string, i, None, None);
    }

    return if Some(strict) { None } else { Some(string.len()) };
}