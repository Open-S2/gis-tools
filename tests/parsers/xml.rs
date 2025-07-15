#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::parsers::{
        NO_INDEX, XMLOptions, XMLPath, XMLPathItem, XMLStep, XMLTag, XMLTagItem,
        xml_count_substring, xml_find_tag_by_name, xml_find_tag_by_path, xml_find_tags_by_path,
        xml_get_attribute, xml_index_of_match, xml_index_of_match_end, xml_remove_comments,
        xml_remove_tags_by_name,
    };

    #[test]
    fn test_xml_find_tag_by_name() {
        let xml = "<tag>value</tag>";
        let tag = xml_find_tag_by_name(xml, "tag", None);
        assert!(tag.is_some());
        assert_eq!(tag.unwrap().outer, xml);
    }

    #[test]
    fn test_xml_count_substrings() {
        let xml = "<tag>value</tag>";
        assert_eq!(xml_count_substring(xml, "value"), 1);

        let nested = "<Thing><Thing attr=1></Thing><Thing attr=2></Thing></Thing>";

        assert_eq!(xml_count_substring(nested, "<namespace:name"), 0);
        assert_eq!(xml_count_substring(nested, "<Test"), 0);
        assert_eq!(xml_count_substring(nested, "<Thing"), 3);
        assert_eq!(xml_count_substring(nested, "/Thing>"), 3);
    }

    #[test]
    fn test_xml_remove_tags_by_name() {
        let xml = "<ul><li>A</li><li>B</li></ul>";
        let res = xml_remove_tags_by_name(xml, "li", None);
        assert_eq!(res, "<ul></ul>");
    }

    #[test]
    fn test_xml_remove_comments() {
        let xml = "<!-- comment -->";
        let res = xml_remove_comments(xml);
        assert_eq!(res, "");

        let xml = "<Thing>\n\n<Thing attr=2></Thing>\n</Thing>";
        let res = xml_remove_comments(xml);
        assert_eq!(res, "<Thing>\n\n<Thing attr=2></Thing>\n</Thing>");

        let xml = "<A><!--<B/>--><!--<C/>--></A>";
        let res = xml_remove_comments(xml);
        assert_eq!(res, "<A></A>");
    }

    #[test]
    fn test_xml_index_of_match() {
        let xml = "<Thing>\n\n<Thing attr=2></Thing>\n</Thing>";
        assert_eq!(xml_index_of_match(xml, "<Thing", 0), 0);
        assert_eq!(xml_index_of_match(xml, "<Thing", 1), 9);
        assert_eq!(xml_index_of_match(xml, "attr=", 0), 16);
    }

    #[test]
    fn test_xml_index_of_match_end() {
        let xml = "<Thing>\n\n<Thing attr=2></Thing>\n</Thing>";
        assert_eq!(xml_index_of_match_end(xml, "</Thing>", 0), 30);
        assert_eq!(xml_index_of_match_end(xml, "</Thing>", 1), 30);
        assert_eq!(xml_index_of_match_end(xml, "poopy", 0), NO_INDEX);

        let xml = "<items><item><item></items>";
        let index = xml_index_of_match_end(xml, "[ /]items>", 0);
        assert_eq!(index, xml.len() - 1);
    }

    #[test]
    fn test_xml_find_tags_by_path() {
        let xml = "<tag>A</tag><tag>B</tag><tag>C</tag><tag>D</tag>";
        let path: XMLPath =
            vec![XMLPathItem::XMLStep(XMLStep { name: "tag".into(), index: Some(2) })];
        let tag = xml_find_tags_by_path(xml, &path, None);
        assert_eq!(
            tag,
            vec![XMLTag {
                outer: "<tag>C</tag>".into(),
                inner: Some("C".into()),
                start: 24,
                end: 36
            }]
        );

        let tag = xml_find_tags_by_path(xml, &vec![], None);
        assert_eq!(tag, vec![]);
    }

    #[test]
    fn test_xml_find_tag_by_path() {
        let xml = "<Thing><Thing sub1>A</Thing><Thing sub2>B</Thing></Thing>";
        let path: XMLPath = vec![XMLPathItem::String("Thing".into())];
        let tag = xml_find_tag_by_path(xml, &path, None);
        assert_eq!(
            tag,
            Some(XMLTag {
                outer: "<Thing><Thing sub1>A</Thing>".into(),
                inner: Some("<Thing sub1>A".into()),
                start: 0,
                end: 28
            })
        );
    }

    #[test]
    fn test_xml_get_attribute() {
        let xml = "<Thing attr=2></Thing>";
        let tag: XMLTagItem =
            XMLTagItem::XMLTag(XMLTag { outer: xml.into(), inner: None, start: 0, end: xml.len() });
        assert_eq!(xml_get_attribute(&tag, "attr"), None);

        let xml = "EEEEEEEEEEEEEEEEE";
        let tag: XMLTagItem =
            XMLTagItem::XMLTag(XMLTag { outer: xml.into(), inner: None, start: 0, end: xml.len() });
        assert_eq!(xml_get_attribute(&tag, "attr"), None);

        let tag_str =
            XMLTagItem::String(r#"<tag attr1="value1" attr2='value2'>Inner Content</tag>"#.into());
        let tag_struct = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<tag attr1="value1" attr2='value2'>Inner Content</tag>"#.into(),
            inner: Some("Inner Content".into()),
            start: 0,
            end: 55,
        });

        assert_eq!(xml_get_attribute(&tag_str, "attr1"), Some("value1".into()));
        assert_eq!(xml_get_attribute(&tag_str, "attr2"), Some("value2".into()));
        assert_eq!(xml_get_attribute(&tag_str, "nonexistent"), None);

        assert_eq!(xml_get_attribute(&tag_struct, "attr1"), Some("value1".into()));
        assert_eq!(xml_get_attribute(&tag_struct, "attr2"), Some("value2".into()));
        assert_eq!(xml_get_attribute(&tag_struct, "nonexistent"), None);
    }

    #[test]
    fn test_xml_find_tags_by_path_nested_true() {
        let xml = r#"
            <root>
                <parent>
                    <child id="1">A</child><child id="2"><child id="3">B</child></child>
                </parent>
            </root>
        "#;

        let path = vec![XMLPathItem::String("parent".into()), XMLPathItem::String("child".into())];

        let options = Some(XMLOptions { nested: Some(true), ..Default::default() });

        let tags = xml_find_tags_by_path(xml, &path, options);

        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0].outer, r#"<child id="1">A</child>"#);
        assert_eq!(tags[1].outer, r#"<child id="2"><child id="3">B</child></child>"#);
        assert_eq!(tags[2].outer, r#"<child id="3">B</child>"#);
    }

    #[test]
    fn test_xml_find_tags_by_path_nested_false() {
        let xml = r#"
            <root>
                <parent>
                    <child id="1">A</child>
                    <child id="2"><child id="3">B</child></child>
                </parent>
            </root>
        "#;

        let path = vec![XMLPathItem::String("parent".into()), XMLPathItem::String("child".into())];

        let options = Some(XMLOptions { nested: Some(false), ..Default::default() });

        let tags = xml_find_tags_by_path(xml, &path, options);

        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].outer, r#"<child id="1">A</child>"#);
        assert_eq!(tags[1].outer, r#"<child id="2"><child id="3">B</child>"#);
    }

    #[test]
    fn test_xml_remove_comments_with_invalid_exclamation() {
        let xml = r#"
        <root>
            <!DOCTYPE html> 
            <data>Hello</data>
        </root>
    "#;

        let result = xml_remove_comments(xml);

        // The <!DOCTYPE html> should remain because it's not a valid comment
        let expected = r#"
        <root>
            <!DOCTYPE html> 
            <data>Hello</data>
        </root>
    "#;

        assert_eq!(result, expected);
    }
}
