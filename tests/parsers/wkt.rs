#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::parsers::{WKTParser, WKTValue, parse_wkt_object};

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

    #[test]
    fn test_weird_ending_bug() {
        let wkt_str = "TEST[ITEM[\"a\", \"b\"], ITEM[\"c\", \"d\"], ITEM[\"e\", \"f\"]]\u{0000}";
        let _wkt_obj = parse_wkt_object(wkt_str);
    }
}
