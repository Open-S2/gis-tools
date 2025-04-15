use alloc::{format, string::String, vec, vec::Vec};
use regex::Regex;

static NO_INDEX: usize = usize::MAX;

/// XMLOptions for xml parsing
#[derive(Debug, Default, Clone)]
pub struct XMLOptions {
    // /// Set true if you want debug info reported
    // pub debug: Option<bool>,
    /// Set the start index
    pub start_index: Option<usize>,
    /// Set true if you want to parse nested tags
    pub nested: Option<bool>,
    /// Set true if you want to return on the first match
    pub return_on_first: Option<bool>,
}

/// A Tag is a pair of an inner and an outer strings with their indexes
#[derive(Debug, Default, Clone, PartialEq)]
pub struct XMLTag {
    /// The inner string
    pub inner: Option<String>,
    /// The outer string
    pub outer: String,
    /// The start index of the tag
    pub start: usize,
    /// The end index of the tag
    pub end: usize,
}
/// A PathItem is a String or a Step
#[derive(Debug, Clone, PartialEq)]
pub enum XMLTagItem {
    /// A String item
    String(String),
    /// A Step item
    XMLTag(XMLTag),
}
/// A Step is a name and an index
#[derive(Debug, Default, Clone, PartialEq)]
pub struct XMLStep {
    /// The name of the step
    pub name: String,
    /// The index of the step
    pub index: Option<usize>,
}
/// A PathItem is a String or a Step
#[derive(Debug, Clone, PartialEq)]
pub enum XMLPathItem {
    /// A String item
    String(String),
    /// A Step item
    XMLStep(XMLStep),
}
/// A Path is an array of Steps or Strings
pub type XMLPath = Vec<XMLPathItem>;

/// Count the number of times a substring appears in a string
pub fn xml_count_substring(string: &str, substring: &str) -> usize {
    let re = Regex::new(substring).unwrap();
    re.find_iter(string).count()
}

/// Find the first tag with the given name
pub fn xml_find_tag_by_name(
    xml: &str,
    tag_name: &str,
    options: Option<XMLOptions>,
) -> Option<XMLTag> {
    let options = options.unwrap_or_default();
    let nested = options.nested == Some(true);

    let start_index = options.start_index.unwrap_or(0);

    // Find the starting index of the tag
    let start = xml_index_of_match(xml, &format!("<{tag_name}[ \n>/]"), start_index);
    if start == NO_INDEX {
        return None;
    }

    let after_start = &xml[start + tag_name.len()..]; // Slice correctly here

    let mut relative_end = xml_index_of_match_end(after_start, "^[^<]*[ /]>", 0);
    let rel_end_char = after_start.chars().nth(relative_end - 1).unwrap_or('\0');
    let self_closing = relative_end != NO_INDEX && rel_end_char == '/';

    if !self_closing {
        // check if tag has subtags with the same name
        if nested {
            let mut start_index = 0;
            let mut openings = 1;
            let mut closings = 0;
            while {
                relative_end =
                    xml_index_of_match_end(after_start, &format!("[ /]{}>", tag_name), start_index);
                relative_end != NO_INDEX
            } {
                let clip = &after_start[start_index..relative_end + 1];
                openings += xml_count_substring(clip, &format!("<{}[ \n\t>]", tag_name));
                closings += xml_count_substring(clip, &format!("</{}>", tag_name));
                // we can't have more openings than closings
                if closings >= openings {
                    break;
                }
                start_index = relative_end;
            }
        } else {
            relative_end = xml_index_of_match_end(after_start, &format!("[ /]{}>", tag_name), 0);
        }
    }

    let end = start + tag_name.len() + relative_end + 1;
    if end == NO_INDEX {
        return None;
    }

    let outer = &xml[start..end]; // Get the full outer tag

    // Extract inner text if it's not self-closing
    let inner: Option<String> = if self_closing {
        None
    } else {
        let start_pos = outer.find('>').unwrap_or(0);
        let end_pos = outer.rfind('<').unwrap_or(outer.len());
        Some(outer[start_pos + 1..end_pos].into())
    };

    Some(XMLTag { inner, outer: outer.into(), start, end })
}

/// Find the first tag with the given path
pub fn xml_find_tag_by_path(
    xml: &str,
    path: &XMLPath,
    options: Option<XMLOptions>,
) -> Option<XMLTag> {
    let found = xml_find_tags_by_path(
        xml,
        path,
        Some(XMLOptions { return_on_first: Some(true), ..options.unwrap_or_default() }),
    );

    found.into_iter().next()
}

/// Find all tags with the given name
/// @param xml - the xml string
/// @param tag_name - the tag name
/// @param options - user defined options
/// @returns all tags with the given name
pub fn xml_find_tags_by_name(
    xml: &str,
    tag_name: &str,
    options: Option<XMLOptions>,
) -> Vec<XMLTag> {
    let options = options.unwrap_or_default();
    let nested = options.nested.unwrap_or(true);
    let mut start_index = options.start_index.unwrap_or(0);
    let mut tags = vec![];
    loop {
        let tag = xml_find_tag_by_name(
            xml,
            tag_name,
            Some(XMLOptions { start_index: Some(start_index), ..options }),
        );
        if let Some(tag) = tag {
            if nested {
                start_index = tag.start + 1 + tag_name.len();
            } else {
                start_index = tag.end;
            }
            tags.push(tag);
        } else {
            break;
        }
    }

    tags
}

/// Find all tags with the given path
pub fn xml_find_tags_by_path(
    xml: &str,
    path: &XMLPath,
    options: Option<XMLOptions>,
) -> Vec<XMLTag> {
    let options = options.unwrap_or_default();
    let return_on_first = options.return_on_first.unwrap_or(false);

    if path.is_empty() {
        return vec![];
    }

    // Extract the first path step
    let path0 = match &path[0] {
        XMLPathItem::String(name) => XMLStep { name: name.clone(), index: None },
        XMLPathItem::XMLStep(step) => step.clone(),
    };

    // Find initial tags
    let mut tags = xml_find_tags_by_name(
        xml,
        &path0.name,
        Some(XMLOptions { nested: Some(false), ..options }),
    );

    // Apply index filtering if present
    if let Some(index) = path0.index {
        tags = tags.get(index).cloned().into_iter().collect();
    }

    let path = &path[1..];

    for (path_index, part) in path.iter().enumerate() {
        let part = match part {
            XMLPathItem::String(name) => XMLStep { name: name.clone(), index: None },
            XMLPathItem::XMLStep(step) => step.clone(),
        };

        let mut all_sub_tags = Vec::new();

        for tag in &tags {
            let mut sub_tags = xml_find_tags_by_name(
                &tag.outer,
                &part.name,
                Some(XMLOptions { start_index: Some(1), ..options }),
            );

            // Adjust tag start positions
            for sub_tag in &mut sub_tags {
                sub_tag.start += tag.start;
                sub_tag.end += tag.start;
            }

            // Early return if return_on_first is set
            if return_on_first && path_index == path.len() - 1 && !sub_tags.is_empty() {
                return vec![sub_tags.remove(0)];
            }

            all_sub_tags.extend(sub_tags);
        }

        tags = all_sub_tags;

        // Apply index filtering at each step if present
        if let Some(index) = part.index {
            tags = tags.get(index).cloned().into_iter().collect();
        }
    }

    tags
}

/// Get the value of an attribute
pub fn xml_get_attribute(tag: &XMLTagItem, attribute_name: &str) -> Option<String> {
    let xml = match tag {
        XMLTagItem::String(s) => s,
        XMLTagItem::XMLTag(t) => &t.outer,
    };

    // Only search for attributes in the opening tag
    if let Some(end) = xml.find('>') {
        let opening = &xml[..=end];

        let quote_chars = ['"', '\''];
        for &quote in &quote_chars {
            let pattern = format!(r#"{}={}([^{}]*){}"#, attribute_name, quote, quote, quote);
            let re = Regex::new(&pattern).ok()?;
            if let Some(captures) = re.captures(opening) {
                return captures.get(1).map(|m| m.as_str().into());
            }
        }
    }
    None
}

/// Find the index of the last match
pub fn xml_index_of_match_end(xml: &str, pattern: &str, start_index: usize) -> usize {
    // let mtch: Vec<(usize, &str)> = xml[start_index..].match_indices(pattern).collect();
    let re = Regex::new(pattern).unwrap();
    let mtch: Vec<(usize, &str)> = re
        .captures_iter(&xml[start_index..])
        .map(|cap| (cap.get(0).unwrap().start(), cap.get(0).unwrap().as_str()))
        .collect();
    if !mtch.is_empty() { start_index + mtch[0].0 + mtch[0].1.len() - 1 } else { NO_INDEX }
}

/// Find the index of the first match
/// @param xml - the xml string
/// @param pattern - the pattern
/// @param start_index - the start index
/// @returns the index of the first match
pub fn xml_index_of_match(xml: &str, pattern: &str, start_index: usize) -> usize {
    // let mtch: Vec<(usize, &str)> = xml[start_index..].match_indices(pattern).collect();
    let re = Regex::new(pattern).unwrap();
    let mtch: Vec<(usize, &str)> = re
        .captures_iter(&xml[start_index..])
        .map(|cap| (cap.get(0).unwrap().start(), cap.get(0).unwrap().as_str()))
        .collect();
    if !mtch.is_empty() { start_index + mtch[0].0 } else { NO_INDEX }
}

/// Remove comments
pub fn xml_remove_comments(xml: &str) -> String {
    // return xml.replace(/<!--[^]*-->/g, '');
    let mut result = String::with_capacity(xml.len());
    let mut inside_comment = false;
    let mut chars = xml.chars().peekable();

    while let Some(c) = chars.next() {
        if inside_comment {
            if c == '-' && chars.peek() == Some(&'-') {
                chars.next(); // Consume second '-'
                if chars.peek() == Some(&'>') {
                    chars.next(); // Consume '>'
                    inside_comment = false;
                }
            }
        } else if c == '<' && chars.peek() == Some(&'!') {
            let mut temp_iter = chars.clone();
            temp_iter.next(); // Consume '!'
            if temp_iter.next() == Some('-') && temp_iter.next() == Some('-') {
                inside_comment = true;
                chars.next(); // Consume '-'
                chars.next(); // Consume '-'
            } else {
                result.push('<');
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Remove tags
pub fn xml_remove_tags_by_name(xml: &str, tag_name: &str, options: Option<XMLOptions>) -> String {
    let mut res: String = xml.into();
    loop {
        let tag = xml_find_tag_by_name(&res, tag_name, options.as_ref().cloned());
        if let Some(tag) = tag {
            res = format!("{}{}", &res[0..tag.start], &res[tag.end..]);
        } else {
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

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
