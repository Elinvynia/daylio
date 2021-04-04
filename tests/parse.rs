use daylio::parse;

#[test]
fn test_parse() {
    parse("tests/test_data.csv").unwrap();
}
