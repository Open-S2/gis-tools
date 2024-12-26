import type { Pbf as Protobuf } from '../..';

/**
 * An internationalized message containing per-language versions of a snippet of
 * text or a URL.
 * One of the strings from a message will be picked up. The resolution proceeds
 * as follows:
 * 1. If the UI language matches the language code of a translation,
 *    the first matching translation is picked.
 * 2. If a default UI language (e.g., English) matches the language code of a
 *    translation, the first matching translation is picked.
 * 3. If some translation has an unspecified language code, that translation is
 *    picked.
 */
export class GTFSRealtimeTranslatedString {
  /** At least one translation must be provided. */
  translations: GTFSRealtimeTranslation[] = []; // 1 [message]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readTranslations, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param translations - The translations to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readTranslations(tag: number, translations: GTFSRealtimeTranslatedString, pbf: Protobuf): void {
    if (tag === 1)
      translations.translations.push(new GTFSRealtimeTranslation(pbf, pbf.readVarint() + pbf.pos));
    else throw new Error(`GTFSRealtimeTranslatedString: Unexpected tag: ${tag}`);
  }
}

/** The translations field of a GTFSRealtimeTranslatedString */
export class GTFSRealtimeTranslation {
  // A UTF-8 string containing the message.
  text!: string; // 1 [string]
  // BCP-47 language code. Can be omitted if the language is unknown or if
  // no i18n is done at all for the feed. At most one translation is
  // allowed to have an unspecified language tag.
  language?: string; // 2 [string]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readTranslation, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param trans - The trans to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readTranslation(tag: number, trans: GTFSRealtimeTranslation, pbf: Protobuf): void {
    if (tag === 1) trans.text = pbf.readString();
    else if (tag === 2) trans.language = pbf.readString();
    else throw new Error(`GTFSRealtimeTranslation: Unexpected tag: ${tag}`);
  }
}
