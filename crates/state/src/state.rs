use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const EDITOR_STATE_CONFIG: &'static str = r#"
export interface EditorStateConfig {
    doc?: string | Text;
}
"#;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "EditorStateConfig")]
    pub type EditorStateConfig;
}
