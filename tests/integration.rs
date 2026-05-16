use step_id::id;

#[test]
fn identical_inputs_identical_ids() {
    assert_eq!(id("r", 0, "k"), id("r", 0, "k"));
}

#[test]
fn run_id_changes_id() {
    assert_ne!(id("r1", 0, "k"), id("r2", 0, "k"));
}

#[test]
fn index_changes_id() {
    assert_ne!(id("r", 0, "k"), id("r", 1, "k"));
}

#[test]
fn kind_changes_id() {
    assert_ne!(id("r", 0, "tool_call"), id("r", 0, "llm_response"));
}

#[test]
fn id_is_16_hex_chars() {
    let i = id("any", 42, "kind");
    assert_eq!(i.len(), 16);
    assert!(i.chars().all(|c| c.is_ascii_hexdigit()));
}
