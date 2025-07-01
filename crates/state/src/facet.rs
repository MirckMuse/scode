use std::process::Output;
use crate::state::EditorState;
use crate::transaction::Transaction;

// facet id；
static NEXT_ID: usize = 0;

pub enum SlotStatus {
    Unresolved = 0,
    Changed = 1,
    Computed = 2,
    Computing = 4,
}

pub trait DynamicSlot {
    fn create(state: EditorState) -> SlotStatus;

    fn update(state: EditorState, tr: Transaction) -> SlotStatus;

    fn reconfigure(state: EditorState, old_state: EditorState) -> SlotStatus;
}

pub struct Configuration {
    #[readonly]
    pub status_template: Vec<SlotStatus>,
}

impl Configuration {
    fn new() -> Self {
        let status = Vec::new();

        return Configuration {
            status_template: status
        };
    }
}

pub fn ensure_address(state: &mut EditorState, address: usize) -> SlotStatus {
    if address & 1 {
        return SlotStatus::Computed;
    }

    let index = address >> 1;

    return match state.status[index] {
        SlotStatus::Computed => SlotStatus::Computed,
        SlotStatus::Computing => {
            panic!("Cyclic dependency between fields and/or facets")
        }
        _ => {
            state.status[index] = SlotStatus::Computing;
            let changed = state.compute_slot(state, state.config.dynamicSlots[index]);

            return match changed {
                None => SlotStatus::Computed,
                _changed => {
                    state.status[index] = SlotStatus::Computed | _changed;
                    return SlotStatus::Computed | _changed;
                }
            };
        }
    };
}

pub fn get_address(state: EditorState, address: usize) {
    if address & 1 {
        return state.config.static_values[address >> 1];
    }

    return state.values[address >> 1];
}

pub struct Facet<Input, Output> {
    id: usize,

    default_output: Output,

    extensions: Option<Extension>,
}

impl<Input, Output> Facet<Input, Output> {
    pub fn new<Combine, CompareInput, CompareOutput>(
        combine: Combine,
        compare_input: CompareInput,
        compare_output: CompareOutput,
    ) -> Facet<Input, Output> {}
}