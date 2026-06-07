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

#[test]
fn id_is_stable_golden_value() {
    // The crate's whole purpose is that the same (run_id, index, kind)
    // yields the same id across versions. This golden value is the first
    // 16 hex chars of SHA-256("run-1\0" + "0" + "\0tool_call"); changing
    // the id derivation would break every stored key, so pin it here.
    assert_eq!(id("run-1", 0, "tool_call"), "d9aca39769c8e3d2");
}

#[test]
fn empty_fields_are_handled() {
    let i = id("", 0, "");
    assert_eq!(i.len(), 16);
    assert!(i.chars().all(|c| c.is_ascii_hexdigit()));
    // Empty fields are distinguished from a non-empty run_id with the
    // same concatenation, thanks to the null separators.
    assert_ne!(id("", 0, "ab"), id("a", 0, "b"));
}
