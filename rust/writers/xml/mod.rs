use alloc::collections::BTreeMap;

/// Create an XML element given the tag name and attributes
///
/// ## Parameters
/// - `tag_name`: the tag name
/// - `attributes`: all the key-value pairs to add to the xml block
/// - `inner_content`: an array of pre-encoded XML content strings
///
/// ## Returns
/// An XML string result
pub fn build_xml_element(
    tag_name: &str,
    attributes: &BTreeMap<String, String>,
    inner_content: &[String],
) -> String {
    // Format the attributes: "key1="value1" key2="value2""
    let attr_string = attributes
        .iter()
        .map(|(key, value)| format!(r#"{}="{}""#, key, value))
        .collect::<Vec<String>>()
        .join(" ");

    let space = if !attr_string.is_empty() { " " } else { "" };

    if !inner_content.is_empty() {
        // Flatten all lines of content to inspect them
        let all_lines: Vec<&str> =
            inner_content.iter().flat_map(|content| content.lines()).collect();
        // If it's just a single line of text, keep it inline
        if all_lines.len() == 1 {
            return format!(
                "<{}{}{}>{}</{}>",
                tag_name, space, attr_string, all_lines[0], tag_name
            );
        }
        // Otherwise, indent multiline children by exactly 1 tab
        let formatted_content =
            all_lines.iter().map(|line| format!("\t{}", line)).collect::<Vec<String>>().join("\n");

        return format!(
            "<{}{}{}>\n{}\n</{}>",
            tag_name, space, attr_string, formatted_content, tag_name
        );
    }

    format!("<{}{}{} />", tag_name, space, attr_string)
}
