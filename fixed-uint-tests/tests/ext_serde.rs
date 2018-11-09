#[macro_use]
extern crate proptest;
extern crate numext_fixed_uint;
extern crate numext_fixed_uint_tests as nfuint_tests;
extern crate serde_json;

use nfuint_tests::props;
use proptest::prelude::any;

proptest! {
    #[test]
    fn with_serde_defun_public(ref le in any::<props::U256LeBytes>()) {
        let val: numext_fixed_uint::U256 = le.into();
        let json = serde_json::to_string(&val);
        assert!(json.is_ok());
        let json = json.unwrap();
        let y = serde_json::from_str(&json);
        assert!(y.is_ok());
        assert_eq!(val, y.unwrap());
    }
}
