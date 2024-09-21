/** Options for xml parsing */
export interface Options {
  debug?: boolean;
  startIndex?: number;
  nested?: boolean;
  returnOnFirst?: boolean;
}

/** A Tag is a pair of an inner and an outer strings with their indexes */
export type Tag = { inner: null | string; outer: string; start: number; end: number };
/** A Step is a name and an index */
export type Step = { name: string; index?: number | undefined | null };
/** A Path is an array of Steps or Strings */
export type Path = Array<string | Step> | ReadonlyArray<string | Step>;

/**
 * @param string - the string
 * @param substring - the substring
 * @returns the number of times the substring appears in the string
 */
export function countSubstring(string: string, substring: string): number {
  const pattern = new RegExp(substring, 'g');
  const match = string.match(pattern);
  return match ? match.length : 0;
}

/**
 * @param xml - the xml string
 * @param tagName - the tag name
 * @param options - user defined options
 * @returns the first tag with the given name
 */
export function findTagByName(xml: string, tagName: string, options?: Options): Tag | undefined {
  const debug = (options && options.debug) ?? false;
  const nested = !(options && options.nested === false);

  const startIndex = (options && options.startIndex) ?? 0;

  if (debug) console.info('[xml-utils] starting findTagByName with', tagName, ' and ', options);

  const start = indexOfMatch(xml, `<${tagName}[ \n>/]`, startIndex);
  if (debug) console.info('[xml-utils] start:', start);
  if (start === -1) return undefined;

  const afterStart = xml.slice(start + tagName.length);

  let relativeEnd = indexOfMatchEnd(afterStart, '^[^<]*[ /]>', 0);

  const selfClosing = relativeEnd !== -1 && afterStart[relativeEnd - 1] === '/';
  if (debug) console.info('[xml-utils] selfClosing:', selfClosing);

  if (selfClosing === false) {
    // check if tag has subtags with the same name
    if (nested) {
      let startIndex = 0;
      let openings = 1;
      let closings = 0;
      while (
        (relativeEnd = indexOfMatchEnd(afterStart, '[ /]' + tagName + '>', startIndex)) !== -1
      ) {
        const clip = afterStart.substring(startIndex, relativeEnd + 1);
        openings += countSubstring(clip, '<' + tagName + '[ \n\t>]');
        closings += countSubstring(clip, '</' + tagName + '>');
        // we can't have more openings than closings
        if (closings >= openings) break;
        startIndex = relativeEnd;
      }
    } else {
      relativeEnd = indexOfMatchEnd(afterStart, '[ /]' + tagName + '>', 0);
    }
  }

  const end = start + tagName.length + relativeEnd + 1;
  if (debug) console.info('[xml-utils] end:', end);
  if (end === -1) return undefined;

  const outer = xml.slice(start, end);
  // tag is like <gml:identifier codeSpace="OGP">urn:ogc:def:crs:EPSG::32617</gml:identifier>

  let inner;
  if (selfClosing) {
    inner = null;
  } else {
    inner = outer.slice(outer.indexOf('>') + 1, outer.lastIndexOf('<'));
  }

  return { inner, outer, start, end };
}

/**
 * @param xml - the xml string
 * @param path - the path
 * @param options - user defined options
 * @returns the first tag with the given path
 */
export function findTagByPath(xml: string, path: Path, options?: Options): Tag | undefined {
  const debug = (options && options.debug) ?? false;
  const found = findTagsByPath(xml, path, { debug, returnOnFirst: true });
  if (Array.isArray(found) && found.length === 1) return found[0];
  else return undefined;
}

/**
 * @param xml - the xml string
 * @param tagName - the tag name
 * @param options - user defined options
 * @returns all tags with the given name
 */
export function findTagsByName(xml: string, tagName: string, options?: Options): Tag[] {
  const tags = [];
  const debug = (options && options.debug) ?? false;
  const nested = options && typeof options.nested === 'boolean' ? options.nested : true;
  let startIndex = (options && options.startIndex) ?? 0;
  let tag;
  while ((tag = findTagByName(xml, tagName, { debug, startIndex }))) {
    if (nested) {
      startIndex = tag.start + 1 + tagName.length;
    } else {
      startIndex = tag.end;
    }
    tags.push(tag);
  }
  if (debug) console.info('findTagsByName found', tags.length, 'tags');
  return tags;
}

/**
 * @param xml - the xml string
 * @param path - the path
 * @param options - user defined options
 * @returns all tags with the given path
 */
export function findTagsByPath(xml: string, path: Path, options?: Options): Tag[] {
  const debug = (options && options.debug) ?? false;
  if (debug) console.info('[xml-utils] starting findTagsByPath with: ', xml.substring(0, 500));
  const returnOnFirst = (options && options.returnOnFirst) ?? false;

  if (Array.isArray(path) === false) throw new Error('[xml-utils] path should be an array');

  const path0 = typeof path[0] === 'string' ? { name: path[0] } : path[0];
  let tags = findTagsByName(xml, path0.name, { debug, nested: false });
  if (typeof tags !== 'undefined' && typeof path0.index === 'number') {
    if (typeof tags[path0.index] === 'undefined') {
      tags = [];
    } else {
      tags = [tags[path0.index]];
    }
  }
  if (debug) console.info('first tags are:', tags);

  path = path.slice(1);

  for (let pathIndex = 0; pathIndex < path.length; pathIndex++) {
    const part: { name: string } | Step =
      typeof path[pathIndex] === 'string'
        ? { name: path[pathIndex] as string }
        : (path[pathIndex] as Step);
    if (debug) console.info('part.name:', part.name);
    let allSubTags: Tag[] = [];
    for (let tagIndex = 0; tagIndex < tags.length; tagIndex++) {
      const tag = tags[tagIndex];
      const subTags = findTagsByName(tag.outer, part.name, {
        debug,
        startIndex: 1,
      });

      if (debug) console.info('subTags.length:', subTags.length);
      if (subTags.length > 0) {
        subTags.forEach((subTag) => {
          subTag.start += tag.start;
          subTag.end += tag.start;
        });
        if (returnOnFirst && pathIndex === path.length - 1) return [subTags[0]];
        allSubTags = allSubTags.concat(subTags);
      }
    }
    tags = allSubTags;
    if ('index' in part && typeof part.index === 'number') {
      if (typeof tags[part.index] === 'undefined') {
        tags = [];
      } else {
        tags = [tags[part.index]];
      }
    }
  }
  return tags;
}

/**
 * @param tag - the tag
 * @param attributeName - the attribute name
 * @param options - user defined options
 * @returns the attribute value
 */
export function getAttribute(
  tag: string | Tag,
  attributeName: string,
  options?: Options,
): string | undefined {
  const debug = (options && options.debug) ?? false;
  if (debug) console.info('[xml-utils] getting ' + attributeName + ' in ' + tag);

  const xml = typeof tag === 'object' ? tag.outer : tag;

  // only search for attributes in the opening tag
  const opening = xml.slice(0, xml.indexOf('>') + 1);

  const quotechars = ['"', "'"];
  for (let i = 0; i < quotechars.length; i++) {
    const char = quotechars[i];
    const pattern = attributeName + '\\=' + char + '([^' + char + ']*)' + char;
    if (debug) console.info('[xml-utils] pattern:', pattern);

    const re = new RegExp(pattern);
    const match = re.exec(opening);
    if (debug) console.info('[xml-utils] match:', match);
    if (match) return match[1];
  }
}

/**
 * @param xml - the xml string
 * @param pattern - the pattern
 * @param startIndex - the start index
 * @returns the index of the last match
 */
export function indexOfMatchEnd(xml: string, pattern: string, startIndex: number): number {
  const re = new RegExp(pattern);
  const match = re.exec(xml.slice(startIndex));
  if (match) return startIndex + match.index + match[0].length - 1;
  else return -1;
}

/**
 * @param xml - the xml string
 * @param pattern - the pattern
 * @param startIndex - the start index
 * @returns the index of the first match
 */
export function indexOfMatch(xml: string, pattern: string, startIndex: number): number {
  const re = new RegExp(pattern);
  const match = re.exec(xml.slice(startIndex));
  if (match) return startIndex + match.index;
  else return -1;
}

/**
 * @param xml - the xml string
 * @returns the xml without comments
 */
export function removeComments(xml: string): string {
  return xml.replace(/<!--[^]*-->/g, '');
}

/**
 * @param xml - the xml string
 * @param tagName - the tag name
 * @param options - user defined options
 * @returns the xml without the given tag
 */
export function removeTagsByName(xml: string, tagName: string, options?: Options) {
  const debug = (options && options.debug) ?? false;
  let tag;
  while ((tag = findTagByName(xml, tagName, { debug }))) {
    xml = xml.substring(0, tag.start) + xml.substring(tag.end);
    if (debug) console.info('[xml-utils] removed:', tag);
  }
  return xml;
}
