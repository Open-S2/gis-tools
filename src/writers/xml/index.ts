/**
 * Create an XML element given the tag name and attributes
 *
 * @param tagName - the tag name
 * @param attributes - all the key-value pairs to add to the xml block
 * @param innerContent - an array of pre-encoded XML content strings
 * @returns An XML string result
 */
export function buildXmlElement(
  tagName: string,
  attributes: Record<string, string | number | boolean> = {},
  innerContent: string[] = [],
): string {
  const attrString = Object.entries(attributes)
    .map(([key, value]) => `${key}="${value}"`)
    .join(' ');
  const space = attrString.length > 0 ? ' ' : '';

  if (innerContent.length > 0) {
    // Flatten all lines of content to inspect them
    const allLines = innerContent.flatMap((content) => content.split('\n'));
    // If it's just a single line of text, keep it inline
    if (allLines.length === 1) {
      return `<${tagName}${space}${attrString}>${allLines[0]}</${tagName}>`;
    }
    // Otherwise, indent multiline children by exactly 1 tab
    const formattedContent = allLines.map((line) => `\t${line}`).join('\n');

    return `<${tagName}${space}${attrString}>\n${formattedContent}\n</${tagName}>`;
  }

  return `<${tagName}${space}${attrString} />`;
}
