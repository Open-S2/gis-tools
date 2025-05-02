use super::clean_string;
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

/// WKT value or array of values
#[derive(Debug, Clone, PartialEq)]
pub enum WKTValue {
    /// A string
    String(String),
    /// A collection of sub WKT values
    Array(Vec<WKTValue>),
}
impl WKTValue {
    /// Pull out the string if it exists
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            WKTValue::String(s) => s.clone(),
            WKTValue::Array(_) => "".into(),
        }
    }
    /// To Float
    pub fn to_float(&self) -> f64 {
        match self {
            WKTValue::String(s) => s.parse().unwrap_or_default(),
            WKTValue::Array(_) => 0.,
        }
    }
    /// Get the Array if it exists
    pub fn to_arr(&self) -> Option<&Vec<WKTValue>> {
        match self {
            WKTValue::Array(arr) => Some(arr),
            WKTValue::String(_) => None,
        }
    }
}
/// WKT object is a collection of WKT values or even nested WKT objects
pub type WKTObject = Vec<WKTValue>;

/// A trait for converting from WKT string (pre-parsed as an object)
pub trait WKTParser: Default {
    /// Converts from WKT
    fn from_wkt(wkt: &WKTValue) -> Self;
}

/// Parses a WKT object
pub fn parse_wkt_object(wkt_str: &str) -> WKTValue {
    let mut res: WKTObject = vec![];
    _parse_wkt_object(wkt_str.into(), &mut res);

    // convert to WKTValue
    WKTValue::Array(res)
}

/// # Parse a WKT object
///
/// ## Arguments
/// - `wkt_str` - WKT string
/// - `res` - collection to store the values
///
/// ## Returns
/// - A sliced WKT string with the parsed values
fn _parse_wkt_object(wkt_str: String, res: &mut WKTObject) -> String {
    let mut wkt_str = wkt_str;

    while !wkt_str.is_empty() {
        // Find indices
        let comma_index = wkt_str.find(',').unwrap_or(usize::MAX);
        let start_bracket_index = wkt_str.find('[').unwrap_or(usize::MAX);
        let end_bracket_index = wkt_str.find(']').unwrap_or(usize::MAX);

        if comma_index < start_bracket_index.min(end_bracket_index) {
            // Store the value before the comma
            let key = wkt_str[..comma_index].to_string();
            if !key.is_empty() {
                res.push(WKTValue::String(clean_string(&key)));
            }
            wkt_str = wkt_str[comma_index + 1..].to_string();
        } else if start_bracket_index < end_bracket_index {
            // Store the object inside brackets
            let key = wkt_str[..start_bracket_index].to_string();
            let mut arr = WKTObject::new();
            wkt_str = _parse_wkt_object(wkt_str[start_bracket_index + 1..].to_string(), &mut arr);
            res.push(WKTValue::String(clean_string(&key)));
            res.push(WKTValue::Array(arr));
        } else {
            // Store the last value if it exists
            if end_bracket_index > 0 {
                let key = clean_string(&wkt_str[..end_bracket_index]);
                if !key.is_empty() {
                    res.push(WKTValue::String(key));
                }
                wkt_str = wkt_str[end_bracket_index + 1..].to_string();
            } else {
                // If no closing bracket, just skip one character
                wkt_str = wkt_str[1..].to_string();
            }
            return wkt_str;
        }
    }

    // Return the remaining string after parsing
    wkt_str
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wkt_object() {
        let wkt_str = r#"TEST[FIRST["a", "b"], SECOND["c", "d"], THIRD["e", "f"]]"#;
        let wkt_obj = parse_wkt_object(wkt_str);

        assert_eq!(
            wkt_obj,
            WKTValue::Array(vec![
                WKTValue::String("TEST".into()),
                WKTValue::Array(vec![
                    WKTValue::String("FIRST".into()),
                    WKTValue::Array(vec![
                        WKTValue::String("a".into()),
                        WKTValue::String("b".into())
                    ]),
                    WKTValue::String("SECOND".into()),
                    WKTValue::Array(vec![
                        WKTValue::String("c".into()),
                        WKTValue::String("d".into())
                    ]),
                    WKTValue::String("THIRD".into()),
                    WKTValue::Array(vec![
                        WKTValue::String("e".into()),
                        WKTValue::String("f".into())
                    ])
                ])
            ])
        );
    }

    #[test]
    fn test_wkt_parser_system() {
        #[derive(Debug, Default, PartialEq)]
        struct Item {
            first: String,
            second: String,
        }
        impl WKTParser for Item {
            fn from_wkt(wkt: &WKTValue) -> Self {
                let mut res = Item::default();
                if let WKTValue::Array(arr) = wkt {
                    res.first = arr.first().map(|s| s.to_string()).unwrap_or_default();
                    res.second = arr.get(1).map(|s| s.to_string()).unwrap_or_default();
                }
                res
            }
        }

        #[derive(Debug, Default, PartialEq)]
        struct Test {
            items: Vec<Item>,
        }
        impl WKTParser for Test {
            fn from_wkt(wkt: &WKTValue) -> Self {
                let mut res = Self::default();
                if let WKTValue::Array(arr) = wkt {
                    for i in (0..arr.len()).step_by(2) {
                        if arr[i].to_string() == "ITEM" {
                            res.items.push(Item::from_wkt(&arr[i + 1]));
                        }
                    }
                }
                res
            }
        }

        let wkt_str = r#"TEST[ITEM["a", "b"], ITEM["c", "d"], ITEM["e", "f"]]"#;
        let wkt_obj = parse_wkt_object(wkt_str);

        let test_item = &wkt_obj.to_arr().unwrap()[1];

        assert_eq!(
            test_item,
            &WKTValue::Array(vec![
                WKTValue::String("ITEM".into()),
                WKTValue::Array(vec![WKTValue::String("a".into()), WKTValue::String("b".into())]),
                WKTValue::String("ITEM".into()),
                WKTValue::Array(vec![WKTValue::String("c".into()), WKTValue::String("d".into())]),
                WKTValue::String("ITEM".into()),
                WKTValue::Array(vec![WKTValue::String("e".into()), WKTValue::String("f".into())])
            ])
        );

        let test = Test::from_wkt(test_item);
        assert_eq!(
            test,
            Test {
                items: vec![
                    Item { first: "a".into(), second: "b".into() },
                    Item { first: "c".into(), second: "d".into() },
                    Item { first: "e".into(), second: "f".into() }
                ]
            }
        );
    }

    #[test]
    fn test_wkt_value() {
        let string_wkt = WKTValue::String("test".into());

        assert_eq!(string_wkt.to_string(), "test".to_string());
        assert_eq!(string_wkt.to_arr(), None);

        let arr_wkt = WKTValue::Array(vec![WKTValue::String("test".into())]);

        assert_eq!(arr_wkt.to_string(), "".to_string());
        assert_eq!(arr_wkt.to_arr(), Some(&vec![WKTValue::String("test".into())]))
    }
}
