import { parseCSVAsRecord } from '../../';

/**
 * # Feed Information
 *
 * **Conditionally Required**
 * Contains information about the dataset itself (publisher, version, etc.).
 * - Required if `translations.txt` is used.
 * - Recommended otherwise.
 */
export class GTFSFeedInfo {
  /**
   * **Required**
   * Full name of the organization that publishes the dataset.
   */
  feedPublisherName: string;
  /**
   * **Required**
   * URL of the dataset publisher's website.
   */
  feedPublisherUrl: string;
  /**
   * **Required**
   * Default language code for the text in this dataset.
   * For multilingual datasets, use "mul" and translations.txt for further detail.
   */
  feedLang: string;
  /**
   * **Optional**
   * Language used if the consumer does not know the rider’s language, often "en".
   */
  defaultLang?: string;
  /**
   * **Recommended**
   * First date of service the dataset covers, in `YYYYMMDD` format.
   */
  feedStartDate?: string;
  /**
   * **Recommended**
   * Last date of service the dataset covers, in `YYYYMMDD` format.
   * Must not precede `feed_start_date` if both are given.
   */
  feedEndDate?: string;
  /**
   * **Recommended**
   * Current version identifier for this GTFS dataset.
   */
  feedVersion?: string;
  /**
   * **Optional**
   * Email address for technical contact about the dataset.
   */
  feedContactEmail?: string;
  /**
   * **Optional**
   * URL for technical contact or support form regarding the dataset.
   */
  feedContactUrl?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.feedPublisherName = data.feed_publisher_name;
    this.feedPublisherUrl = data.feed_publisher_url;
    this.feedLang = data.feed_lang;
    this.defaultLang = data.default_lang;
    this.feedStartDate = data.feed_start_date;
    this.feedEndDate = data.feed_end_date;
    this.feedVersion = data.feed_version;
    this.feedContactEmail = data.feed_contact_email;
    this.feedContactUrl = data.feed_contact_url;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSFeedInfos
 */
export function parseGTFSFeedInfos(input: string): Record<string, GTFSFeedInfo> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSFeedInfo> = {};
  for (const d of data) {
    const fi = new GTFSFeedInfo(d);
    res[fi.feedPublisherName] = fi;
  }

  return res;
}
