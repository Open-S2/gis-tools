use alloc::{format, string::String, vec, vec::Vec};
use regex::Regex;

/// # NO_INDEX means it's not pointing to a location in memory
pub static NO_INDEX: usize = usize::MAX;

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
                    xml_index_of_match_end(after_start, &format!("[ /]{tag_name}>"), start_index);
                relative_end != NO_INDEX
            } {
                let clip = &after_start[start_index..relative_end + 1];
                openings += xml_count_substring(clip, &format!("<{tag_name}[ \n\t>]"));
                closings += xml_count_substring(clip, &format!("</{tag_name}>"));
                // we can't have more openings than closings
                if closings >= openings {
                    break;
                }
                start_index = relative_end;
            }
        } else {
            relative_end = xml_index_of_match_end(after_start, &format!("[ /]{tag_name}>"), 0);
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
            let pattern = format!(r#"{attribute_name}={quote}([^{quote}]*){quote}"#);
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
