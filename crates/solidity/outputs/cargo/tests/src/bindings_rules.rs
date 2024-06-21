use slang_solidity::bindings::Bindings;

#[test]
fn test_bindings_rules_parsing() {
    let result = Bindings::get_graph_builder();
    assert!(result.is_ok());
}
