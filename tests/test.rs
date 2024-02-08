use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Value {
    #[serde(rename = "unused_0")]
    unused0: String,
    unused1: Option<String>,
    unused2: i32,
    unused3: u32,
    unused4: f32,
    unused5: Enum,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
enum Enum {
    #[serde(rename = "u0")]
    U0,
    #[serde(rename = "u1")]
    U1,
}

#[test]
fn from_str() {
    let prop = r"
    # test comment by '#'
    ! test comment by '!'

    # test colon
    unused_0: unused 0

    unused1=
    unused2=2
    unused3=3
    unused4=4.4
    unused5=u0
    unused6=_
    ";

    let v: Value = serde_prop::from_str(prop).unwrap();
    assert_eq!(v.unused0, "unused 0");
    assert_eq!(v.unused1, None);
    assert_eq!(v.unused2, 2_i32);
    assert_eq!(v.unused3, 3_u32);
    assert_eq!(v.unused4, 4.4_f32);
    assert_eq!(v.unused5, Enum::U0)
}

#[test]
fn to_string() {
    let v = Value {
        unused0: "unused 0".to_owned(),
        unused1: None,
        unused2: 2_i32,
        unused3: 3_u32,
        unused4: 4.4_f32,
        unused5: Enum::U0,
    };

    let s = serde_prop::to_string(&v).unwrap();
    assert_eq!(
        s,
        "unused_0=unused 0\nunused1=\nunused2=2\nunused3=3\nunused4=4.4\nunused5=u0"
    );
}
