#![allow(dead_code)]

use std::ops::Index;

const ZERO_WIDTH_JOINER: u32 = 0x200D;

fn is_extending_char(code: usize) -> bool {
    todo!();
    return false;
}

fn is_regional_indicator(code: usize) -> bool {
    return 0x1F1E6 <= code && code <= 0x1F1FF;
}

pub fn find_cluster_break(
    string: &String,
    position: usize,
    forward: Option<bool>,
    include_extending: Option<bool>,
) -> usize {
    let forward = forward.unwrap_or(true);
    let include_extending = include_extending.unwrap_or(true);
    if forward {
        return next_cluster_break(string, position, include_extending);
    }

    return prev_cluster_break(string, position, include_extending);
}

fn next_cluster_break(str: &String, pos: usize, include_extending: bool) -> usize {
    let mut pos = pos;
    if pos == str.len() {
        return pos;
    }

    if pos > 0
        && is_low_surrogate(char_code_at(str, pos))
        && is_high_surrogate(char_code_at(str, pos - 1))
    {
        pos = pos - 1;
    }

    let mut prev = code_point_at(str, pos) as usize;
    pos += code_point_size(prev) as usize;

    while pos < str.len() {
        let next = code_point_at(str, pos) as usize;

        if prev as u32 == ZERO_WIDTH_JOINER
            || next as u32 == ZERO_WIDTH_JOINER
            || include_extending && is_extending_char(next)
        {
            pos = pos + code_point_size(next) as usize;
            prev = next;
        } else if is_regional_indicator(next) {
            let mut count_before = 0;
            let mut i = pos - 2;

            while i >= 0 && is_regional_indicator(code_point_at(str, i) as usize) {
                count_before += 1;
                i -= 2;
            }

            if count_before % 2 == 0 {
                break;
            } else {
                pos += 2;
            }
        } else {
            break;
        }
    }

    return pos;
}

/// 判断是否为低位代理
fn is_low_surrogate(code: u32) -> bool {
    return 0xDC00 <= code && code < 0xE000;
}

/// 判断是否为高位代理
fn is_high_surrogate(code: u32) -> bool {
    return 0xD800 <= code && code < 0xDC00;
}

fn prev_cluster_break(str: &String, pos: usize, include_extending: bool) -> usize {
    let mut pos = pos;
    while pos > 0 {
        let found = next_cluster_break(str, pos - 2, include_extending);

        if found < pos {
            return found;
        }

        pos -= 1;
    }

    return 0;
}

fn char_code_at(str: &String, index: usize) -> u32 {
    return str.chars().nth(index).unwrap() as u32;
}

fn code_point_at(str: &String, index: usize) -> u32 {
    let code0 = char_code_at(str, index);

    if !is_high_surrogate(code0) || index + 1 == str.len() {
        return code0;
    }

    let code1 = char_code_at(str, index + 1);

    if !is_low_surrogate(code1) {
        return code0;
    }

    return ((code0 - 0xD800) << 10) + (code1 - 0xDC00) + 0x10000;
}

fn from_char_code(codes: Vec<usize>) -> String {
    return String::from_utf8(codes.iter().map(|v| *v as u8).collect()).unwrap();
}

fn from_code_point(codes: Vec<usize>) -> String {
    if *codes.index(0) <= 0xFFFFusize {
        return from_char_code(codes);
    }

    let code = *codes.index(0) - 0x10000;

    return from_char_code(vec![(code >> 10) + 0xD800, (code & 1023) + 0xDC00]);
}

/// 根据 code point 判断
fn code_point_size(code: usize) -> u32 {
    return if code < 0x10000 { 1 } else { 2 };
}
