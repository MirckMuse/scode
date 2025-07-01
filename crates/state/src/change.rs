pub struct ChangeDesc {
    sections: Vec<isize>,
}

type IterGapsCallback = fn(pos_a: isize, pos_b: isize, length: isize);
type IterChangeRangesCallback = fn(from_a: isize, to_a: isize, from_b: isize, to_b: isize);

#[derive(Debug)]
#[wasm_bindgen]
pub enum MapMode {
    Simple,
    TrackDel,
    TrackBefore,
    TrackAfter,
}

impl ChangeDesc {
    fn new(selections: Vec<isize>) -> ChangeDesc {
        return ChangeDesc { sections: selections };
    }

    fn create(sections: Vec<isize>) -> ChangeDesc {
        return ChangeDesc::new(sections);
    }

    fn len(&self) -> usize {
        let mut length: usize = 0;
        let mut i = 0;

        while i < self.sections.len() {
            length += self.sections[i];
            i += 2;
        }

        return length;
    }

    fn len_after_change(&self) -> usize {
        let mut length: usize = 0;
        let mut i = 0;


        while i < self.sections.len() {
            let ins = self.sections[i + 1];
            if ins < 0 {
                length += self.sections[i];
            } else {
                length += ins;
            }
            i += 2;
        }

        return length;
    }

    fn is_empty(&self) -> bool {
        return self.sections.is_empty() || self.sections.len() == 2 && self.sections[1] < 0;
    }

    fn iter_gaps(&self, callback: IterGapsCallback) {
        let mut i = 0;
        let mut pos_a = 0;
        let mut pos_b = 0;

        while i < self.sections.len() {
            let (len, ins, new_i) = vec_stepping2(&self.sections, i);
            i = new_i;

            if ins < 0 {
                callback(pos_a, pos_b, len);
                pos_b += len;
            } else {
                pos_b += ins;
            }

            pos_a += len;
        }
    }

    fn iter_change_ranges(&self, callback: IterChangeRangesCallback, individual: Option<bool>) {
        let individual = individual.unwrap_or(false);

        iter_changes(&self, callback, individual)
    }

    fn inverted_desc(&self) -> ChangeDesc {
        let mut sections: Vec<isize> = Vec::new();
        let mut i = 0;
        while i < self.sections.len() {
            let (len, ins, new_i) = vec_stepping2(&self.sections, i);
            i = new_i;

            if ins < 0 {
                sections.push(len);
                sections.push(ins);
            } else {
                sections.push(ins);
                sections.push(len);
            }
        }

        return ChangeDesc::new(sections);
    }

    fn compose_desc(&self, other: ChangeDesc) -> &ChangeDesc {
        if self.is_empty() {
            return &other;
        }

        if other.is_empty() {
            return self;
        }

        return compose_sets(self, other);
    }

    fn map_desc(&self, other: ChangeDesc, before: Option<bool>) -> &ChangeDesc {
        let before = before.unwrap_or(false);

        if other.is_empty() {
            return self;
        }

        return map_set(self, other, before);
    }

    fn map_pos(&self, pos: isize, assoc: Option<isize>, mode: Option<MapMode>) -> Option<isize> {
        let assoc = assoc.unwrap_or(-1);
        let mode = mode.unwrap_or(MapMode::Simple);

        let mut pos_a = 0;
        let mut pos_b = 0;

        let mut i = 0;

        while i < self.sections.len() {
            let (len, ins, new_i) = vec_stepping2(&self.sections, i);
            i = new_i;
            let end_a = pos_a + len;

            if ins < 0 {
                if end_a > pos {
                    return Some(pos_b + pos - pos_a);
                }

                pos_b += len;
            } else {
                if mode != MapMode::Simple &&
                    end_a >= pos &&
                    (
                        mode == MapMode::TrackDel && pos_a < pos && end_a > pos ||
                            mode == MapMode::TrackBefore && pos_a < pos ||
                            mode == MapMode::TrackAfter && pos_a > pos
                    ) {
                    return None;
                }
                if end_a > pos || end_a == pos && assoc < 0 && len != 0 {
                    if pos == pos_a || assoc < 0 {
                        return Some(pos_b);
                    }

                    return Some(pos_b + ins);
                }

                pos_b += ins;
            }

            pos_a = end_a;
        }

        if pos > pos_a {
            panic!("Position {} is out of range for changeset of length {}", pos, pos_a);
        }
        return Some(pos_b);
    }

    fn touches_range(&self, from: isize, to: Option<isize>) -> bool {
        let to = to.unwrap_or(from);

        let mut i = 0;
        let mut pos = 0;

        while i < self.sections.len() && pos <= to {
            let (len, ins, new_i) = vec_stepping2(&self.sections, i);
            i = new_i;

            let end = pos + len;

            if ins >= 0 && pos <= to && end >= from {
                if pos < from && end > to {
                    // TODO: 需要研究怎么传递联合类型。
                    return "cover";
                }

                return true;
            }
        }

        return false;
    }

    fn to_string(&self) -> String {
        let mut string = String::new();

        let mut i = 0;

        while i < self.sections.len() {
            let (len, ins, new_i) = vec_stepping2(&self.sections, i);
            i = new_i;
        }

        return string;
    }
}

fn vec_stepping2<T: Clone>(values: &Vec<T>, start: usize) -> (T, T, usize) {
    let mut i = start;
    let one = values[i + 1].clone();
    i += 1;
    let two = values[i + 1].clone();
    i += 1;

    return (one, two, i);
}


fn iter_changes(desc: &ChangeDesc, callback: IterChangeRangesCallback, individual: bool) {
    todo!("等待实现")
}