use wasm_bindgen::prelude::*;
//use std::cmp;

enum Tree {
    BranchShift = 5,
    Branch = 32,
}

enum Open {
    From = 1,
    To = 2,
}

#[wasm_bindgen]
pub struct Line {
    /// 行终点位置
    to: isize,

    /// 行起始位置
    from: isize,

    /// 当前行的序号
    number: isize,

    /// 行内容
    text: String,
}
//
//impl Line {
//    fn new(from: isize, to: isize, number: isize, text: String) -> Self {
//        return Line { from, to, number, text };
//    }
//
//    fn length(&self) -> isize {
//        return self.to - self.from;
//    }
//}
//
//pub trait TextIterator: Iterator {
//    type Item;
//
//    fn next(&self, skip: Option<isize>) -> Self;
//}
//
//pub enum TextDir {
//    Left,
//    Right,
//}
//
//pub trait Text: Iterator {
//    /// 获取文本长度
//    fn len(&self) -> isize;
//
//    /// 获取当前行号
//    fn lines(&self) -> isize;
//
//    fn line_inner(&self, pos: isize, is_line: bool, line: isize, offset: isize) -> Line;
//
//    /// Retrieve a part of the document as a string
//    fn slice_string(from: isize, to: Option<isize>, line_seq: Option<String>) -> String;
//
//    /// 获取指定位置的描述
//    fn line_at(&self, pos: isize) -> Line {
//        if pos < 0 || pos > self.len() {
//            panic!("Invalid position {} in document of length {}", pos, self.len())
//        }
//
//        return self.line_inner(pos, false, 1, 0);
//    }
//
//    /// If this is a branch node, `children` will hold the `Text`
//    /// objects that it is made up of. For leaf nodes, this holds null.
//    fn children(&self) -> Option<Vec<String>>;
//
//    fn flatten(&self, targets: Vec<String>);
//
//    fn scan_identical(&self, text: &impl Text, dir: TextDir) -> isize;
//
//    fn decompose(&self, from: isize, to: isize, target: Vec<impl Text>, open: isize);
//
//    const EMPTY: dyn Text;
//
//    fn of(text: Vec<String>) -> impl Text {
//        if text.len() == 0 {
//            panic!("A document must have at lease one line");
//        }
//
//        if text.len() == 1 && text[0].len() == 0 {
//            return Self::EMPTY;
//        }
//
//        if text.len() <= Tree::Branch as usize {
//            return TextLeaf::new(text);
//        }
//
//        return TextNode::from(TextLeaf::split(text, Vec::new()));
//    }
//
//    /// 获取指定行号的行描述
//    fn line(&self, line: isize) -> Line {
//        if line < 1 || line > self.lines() {
//            panic!("Invalid line number {} in {}-line document", line, self.lines())
//        }
//
//        return self.line_inner(line, true, 1, 0);
//    }
//
//    /// 指定范围替换文本
//    fn replace(&self, from: isize, to: isize, text: &impl Text) -> impl Text {
//        let (from, to) = adjust_range(text, from, to);
//
//        let parts: Vec<impl Text> = Vec::new();
//
//        // TODO:
//    }
//
//    /// 附加其他文档到当前
//    fn append(&self, text: &impl Text) -> impl Text {
//        return self.replace(self.len(), self.len(), text);
//    }
//
//    fn slice(&self, from: isize, to: Option<isize>) -> impl Text {
//        let (from, to) = adjust_range(self, from, to.unwrap_or(self.len()));
//
//        let parts: Vec<impl Text> = Vec::new();
//
//        self.decompose(from, to, parts, 0);
//
//        return TextNode::from(parts, to - from);
//    }
//
//    fn eq(&self, other: &impl Text) -> bool {
//        if std::ptr::eq(self, other) { return true; }
//
//        if self.len() != other.len() || self.lines() != other.lines() { return false; }
//
//        let start = self.scan_identical(other, TextDir::Right);
//        let end = self.len() - self.scan_identical(other, TextDir::Left);
//
//        return false;
//    }
//}
//
//#[wasm_bindgen]
//pub struct TextLeaf {
//    text: Vec<String>,
//
//    length: isize,
//}
//
//impl TextLeaf {
//    fn new(text: Vec<String>, length: Option<isize>) -> TextLeaf {
//        return TextLeaf {
//            text: text.clone(),
//            length: length.unwrap_or(text_len(text)),
//        };
//    }
//
//    fn lines(&self) -> usize {
//        return self.text.len();
//    }
//
//    fn children() {
//        todo!("等待实现确认")
//    }
//}
//
//impl Text for TextLeaf {
//    fn len(&self) -> isize {
//        todo!()
//    }
//
//    fn lines(&self) -> isize {
//        todo!()
//    }
//
//    fn line_inner(&self, pos: isize, is_line: bool, line: isize, offset: isize) -> Option<Line> {
//        let mut line = line;
//        let mut offset = offset;
//        let mut i = 0;
//        while true {
//            let string = self.text[i].clone();
//            let end = offset + string.lines();
//
//            if (is_line && line >= pos) || (!is_line && end > pos) {
//                return Some(Line::new(offset, end, line, string));
//            }
//
//            offset = end + 1;
//            line = line + 1;
//        }
//
//        return None;
//    }
//
//    fn slice_string(from: isize, to: Option<isize>, line_seq: Option<String>) -> String {
//        todo!()
//    }
//
//    fn children(&self) -> Option<Vec<String>> {
//        todo!()
//    }
//
//    fn flatten(&self, targets: Vec<String>) {
//        todo!()
//    }
//
//    fn scan_identical(&self, text: &impl Text, dir: TextDir) -> isize {
//        todo!()
//    }
//
//    fn decompose(&self, from: isize, to: isize, target: &mut Vec<impl Text>, open: isize) {
//        let text: &TextLeaf;
//
//        if from <= 0 && to >= self.len() {
//            text = self;
//        } else {
//            text = &TextLeaf::new(
//                slice_text(&self.text, Some(from), Some(to)),
//                Some(cmp::min(to, self.len()) - cmp::max(0, from)),
//            );
//        }
//
//        if open == Open::From as isize {
//            let prev = target.pop() as TextLeaf;
//            let joined = append_text(&text.text, &mut prev.text.clone(), Some(0), Some(text.len()));
//
//            if joined.len() <= Tree::Branch as usize {
//                target.push(TextLeaf::new(joined, Some(prev.len() + text.len())))
//            } else {
//                let mid = joined.len() >> 1;
//                target.push(TextLeaf::new(joined[0..mid].to_vec(), None));
//                target.push(TextLeaf::new(joined[mid..].to_vec(), None));
//            }
//        } else {
//            target.push(text.clone())
//        }
//    }
//
//    const EMPTY: dyn Text = ();
//
//    fn replace(&self, from: isize, to: isize, text: &impl Text) -> impl Text {}
//}
//
//#[wasm_bindgen]
//pub struct TextNode {}
//
//impl Text for TextNode {}
//
//struct LineCursor {
//    after_break: bool,
//
//    value: String,
//
//    done: bool,
//
//    inner: dyn TextIterator,
//}
//
//impl LineCursor {
//    fn new(inner: &mut impl TextIterator) -> LineCursor {
//        return LineCursor {
//            after_break: true,
//            value: "".to_string(),
//            done: false,
//            inner: inner,
//        };
//    }
//
//    fn line_break(&self) -> bool {
//        return false;
//    }
//}
//
//impl TextIterator for LineCursor {
//    type Item = String;
//
//    fn next(&self, skip: Option<isize>) -> Self {
//        let skip = skip.unwrap_or(0);
//
//        let text = self.inner.next(Option::from(skip));
//
//        todo!("需要实现字符串的迭代方法")
//    }
//}
//
//
///// 校准范围
//fn adjust_range(text: &impl Text, from: isize, to: isize) -> (isize, isize) {
//    let _from = cmp::max(0, cmp::min(text.len(), from));
//    let _to = cmp::max(_from, cmp::min(text.len(), to));
//    return (_from, _to);
//}
//
//fn text_len(text: Vec<String>) -> isize {
//    let mut len: isize = -1;
//    for lin in text {
//        len = len + lin.len() + 1;
//    }
//    return len;
//}
//
//fn append_text(text: &Vec<String>, target: &mut Vec<String>, from: Option<isize>, to: Option<isize>) -> Vec<String> {
//    let from = from.unwrap_or(0);
//    let to = to.unwrap_or(1e9 as isize);
//
//    let mut pos = 0;
//    let mut i = 0;
//    let mut first = true;
//
//    while i < text.len() && pos <= to {
//        let mut line = text[i].clone();
//        let end = pos + line.len();
//
//        if end >= from {
//            if end > to {
//                line = line[0..(to - pos)]
//            }
//
//            if pos < from {
//                line = line[(from - pos)..]
//            }
//            if first {
//                target[target.len() - 1].push_str(&line);
//                first = false;
//            } else {
//                target.push(line)
//            }
//        }
//        pos = end + 1;
//        i = i + 1;
//    }
//
//    // TODO: 需要解引用
//    return target;
//}
//
//fn slice_text(text: &Vec<String>, from: Option<isize>, to: Option<isize>) -> Vec<String> {
//    let mut target: Vec<String> = Vec::new();
//    target.push("".to_string());
//    return append_text(text, &mut target, from, to);
//}