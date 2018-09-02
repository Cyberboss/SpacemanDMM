extern crate dreammaker as dm;

use dm::lexer::{Quote, FormatFloat};

#[test]
fn strings() {
    assert_eq!(Quote("test").to_string(), r#""test""#);
    // TODO: more tests...
}

#[test]
fn floats() {
    assert_eq!(FormatFloat(0.0).to_string(), "0");
    assert_eq!(FormatFloat(1.0).to_string(), "1");
    assert_eq!(FormatFloat(99999.0).to_string(), "99999");
    assert_eq!(FormatFloat(99999.01).to_string(), "99999");
    assert_eq!(FormatFloat(999999.0).to_string(), "999999");
    assert_eq!(FormatFloat(5.0e6).to_string(), "5e+006");
    assert_eq!(FormatFloat(5000000i32 as f32).to_string(), "5e+006");
    assert_eq!(FormatFloat(9999999.0).to_string(), "1e+007");
    assert_eq!(FormatFloat(9999991.0).to_string(), "9.99999e+006");

    assert_eq!(FormatFloat(1.00005).to_string(), "1.00005");
    assert_eq!(FormatFloat(1.000005).to_string(), "1.00001");

    assert_eq!(FormatFloat(0.000500001).to_string(), "0.000500001");
    assert_eq!(FormatFloat(0.0000500001).to_string(), "5.00001e-005");

    assert_eq!(FormatFloat(std::f32::INFINITY).to_string(), "1.#INF");
    assert_eq!(FormatFloat(-std::f32::INFINITY).to_string(), "-1.#INF");
}
