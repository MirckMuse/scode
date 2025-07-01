use crate::change::*;

pub struct AnnotationType<T>;

impl<T> AnnotationType<T> {
    fn new<T>() -> AnnotationType<T> {
        return AnnotationType {};
    }

    fn of(&self, value: T) -> Annotation<T> {
        return Annotation::new(self, value);
    }
}

pub struct Annotation<'a, T> {
    value: T,

    _type: &'a AnnotationType<T>,

    _is_annotation: bool,
}

impl<T> Annotation<T> {
    fn new(_type: &AnnotationType<T>, value: T) -> Annotation<T> {
        return Annotation { _type, value, _is_annotation: true };
    }

    fn define<T>() -> AnnotationType<T> {
        return AnnotationType::new();
    }
}

type StateEffectMapping<T> = fn(value: T, mapping: ChangeDesc) -> Option<T>;

pub trait StateEffectSpec<T> {
    const map: StateEffectMapping
}

pub struct Transaction {}