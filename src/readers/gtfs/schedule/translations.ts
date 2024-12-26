import { parseCSVAsRecord } from '../../';

/**
 * # Translations
 *
 * **Optional**
 * Provides language-specific translations for text fields in various GTFS tables.
 * Each row defines a single translation for a specific field in a specific language,
 * either targeting a specific record (and possibly sub-record) or matching by field value.
 *
 * **Primary Key**: (table_name, field_name, language, record_id, record_sub_id, field_value)
 */
export class GTFSTranslation {
  /**
   * **Required**
   * The table containing the field to be translated.
   *
   * Allowed values (official spec):
   * - "agency"
   * - "stops"
   * - "routes"
   * - "trips"
   * - "stop_times"
   * - "pathways"
   * - "levels"
   * - "feed_info"
   * - "attributions"
   *
   * Other optional files (calendar, shapes, etc.) may appear for unofficial field translations.
   */
  tableName: string;
  /**
   * **Required**
   * Name of the field within the table that is being translated.
   * Typically text, URL, phone, or email fields.
   */
  fieldName: string;
  /**
   * **Required**
   * ISO language code (e.g., "en", "fr", "mul") for this translation.
   */
  language: string;
  /**
   * **Required**
   * The translated value, matching the type of the original field (text, URL, phone, email).
   */
  translation: string;
  /**
   * **Conditionally Required**
   * Identifies the primary key of the record in the table if the table has a unique ID
   * (e.g., `agency_id`, `stop_id`, `route_id`, `trip_id`, `pathway_id`, `level_id`, `attribution_id`).
   * Required unless `fieldValue` is used or if `tableName=feed_info`.
   */
  recordId?: string;
  /**
   * **Conditionally Required**
   * Secondary key if the table doesn’t have a single unique ID (e.g., stop_sequence for stop_times).
   * Required if `recordId` is used and `tableName=stop_times`. Forbidden otherwise.
   */
  recordSubId?: string;
  /**
   * **Conditionally Required**
   * The exact field value to match for translation if `recordId` and `recordSubId` are not used.
   * Forbidden if `recordId` is defined or if `tableName=feed_info`.
   */
  fieldValue?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.tableName = data.table_name;
    this.fieldName = data.field_name;
    this.language = data.language;
    this.translation = data.translation;
    // These fields are optional/conditional
    this.recordId = data.record_id;
    this.recordSubId = data.record_sub_id;
    this.fieldValue = data.field_value;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Translations
 */
export function parseGTFSTranslations(input: string): GTFSTranslation[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSTranslation(d));
}
