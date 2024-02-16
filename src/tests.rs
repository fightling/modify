use crate::{Modifiable, Modify};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Test {
    #[serde(flatten)]
    value: Modify<usize>,
}

#[test]
fn test_modify() {
    let mut value = Test {
        value: Modify::new(42),
    };
    *value.value = 43;
    assert!(value.value.is_modified());
    let mut writer: Vec<u8> = Vec::new();
    serde_json::to_writer_pretty(&mut writer, &value).unwrap();
    eprintln!("{}", String::from_utf8(writer).unwrap());
}
