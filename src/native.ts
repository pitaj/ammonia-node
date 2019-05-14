// eslint-disable-next-line @typescript-eslint/no-var-requires
const addon = require('../native');

export interface BuilderOptions {
  /**
   * Sets the tags that are allowed.
   *
   * @default [
   *   a, abbr, acronym, area, article, aside, b, bdi,
   *   bdo, blockquote, br, caption, center, cite, code,
   *   col, colgroup, data, dd, del, details, dfn, div,
   *   dl, dt, em, figcaption, figure, footer, h1, h2,
   *   h3, h4, h5, h6, header, hgroup, hr, i, img,
   *   ins, kbd, kbd, li, map, mark, nav, ol, p, pre,
   *   q, rp, rt, rtc, ruby, s, samp, small, span,
   *   strike, strong, sub, summary, sup, table, tbody,
   *   td, th, thead, time, tr, tt, u, ul, var, wbr
   * ]
   */
  tags: string[];
  /**
   * Sets the tags whose contents will be completely removed from the output.
   *
   * Adding tags which are whitelisted in tags or tag_attributes will cause a panic.
   *
   * @default []
   */
  clean_content_tags: string[];
  /**
   * Sets the HTML attributes that are allowed on specific tags.
   *
   * The value is structured as a map from tag names to a set of attribute names.
   *
   * If a tag is not itself whitelisted, adding entries to this map will do nothing.
   *
   * @default {
   *   a =>
   *       href, hreflang
   *   bdo =>
   *       dir
   *   blockquote =>
   *       cite
   *   col =>
   *       align, char, charoff, span
   *   colgroup =>
   *       align, char, charoff, span
   *   del =>
   *       cite, datetime
   *   hr =>
   *       align, size, width
   *   img =>
   *       align, alt, height, src, width
   *   ins =>
   *       cite, datetime
   *   ol =>
   *       start
   *   q =>
   *       cite
   *   table =>
   *       align, char, charoff, summary
   *   tbody =>
   *       align, char, charoff
   *   td =>
   *       align, char, charoff, colspan, headers, rowspan
   *   tfoot =>
   *       align, char, charoff
   *   th =>
   *       align, char, charoff, colspan, headers, rowspan, scope
   *   thead =>
   *       align, char, charoff
   *   tr =>
   *       align, char, charoff
   * }
   */
  tag_attributes: {
    [tag: string]: string[];
  };
  /**
   * Sets the values of HTML attributes that are allowed on specific tags.
   *
   * The value is structured as a map from tag names to a map
   * from attribute names to a set of attribute values.
   *
   * If a tag is not itself whitelisted, adding entries to this map will do nothing.
   *
   * @default {}
   */
  tag_attribute_values: {
    [tag: string]: {
      [attr: string]: string[];
    };
  };
  /**
   * Sets the attributes that are allowed on any tag.
   *
   * @default [lang, title]
   */
  generic_attributes: string[];
  /**
   * Sets the URL schemes permitted on href and src attributes.
   *
   * @default [
   *   bitcoin, ftp, ftps, geo, http, https, im, irc,
   *   ircs, magnet, mailto, mms, mx, news, nntp,
   *   openpgp4fpr, sip, sms, smsto, ssh, tel, url,
   *   webcal, wtai, xmpp
   * ]
   */
  url_schemes: string[];
  /**
   * Configures the behavior for relative URLs: pass-through, resolve-with-base, or deny.
   *
   * @default 'pass-through'
   */
  url_relative: 'deny' | 'pass-through' | ['resolve-with-base', string];
  //  ['custom', (url: string) => string | null];

  // /**
  //  * Allows rewriting of all attributes using a callback.
  //  *
  //  * The callback takes name of the element, attribute and its value.
  //  * Returns `null` to remove the attribute, or a value to use.
  //  *
  //  * Rewriting of attributes with URLs is done before url_relative().
  //  */
  // attribute_filter: (element: string, attribute: string, value: string) => string | null;

  /**
   * Configures a `rel` attribute that will be added on links.
   *
   * If `rel` is in the generic or tag attributes, this must be set to `null`.
   * Common rel values to include:
   *
   *  - `noopener`: This prevents a particular type of XSS attack,
   *   and should usually be turned on for untrusted HTML.
   *  - `noreferrer`: This prevents the browser from sending the
   *   source URL to the website that is linked to.
   *  - `nofollow`: This prevents search engines from using this link for ranking,
   *   which disincentivizes spammers.
   *
   * To turn on rel-insertion, call this function with a space-separated list.
   * Ammonia does not parse rel-attributes;
   * it just puts the given string into the attribute directly.
   *
   * @default 'noopener noreferrer'
   */
  link_rel: string | null;
  /**
   * Sets the CSS classes that are allowed on specific tags.
   *
   * The values is structured as a map from tag names to a set of class names.
   *
   * If the class attribute is itself whitelisted for a tag,
   * then adding entries to this map will cause a panic.
   *
   * @default {}
   */
  allowed_classes: {
    [tag: string]: string[];
  };
  /**
   * Configures the handling of HTML comments.
   *
   * If this option is false, comments will be preserved.
   *
   * @default true
   */
  strip_comments: boolean;
  /**
   * Prefixes all "id" attribute values with a given string.
   * Note that the tag and attribute themselves must still be whitelisted.
   *
   * @default null
   */
  id_prefix: string | null;
}

class CleanerType {
  // eslint-disable-next-line
  public constructor(options: Partial<BuilderOptions>) {}

  /**
   * Sanitizes an HTML fragment in a string according to the configured options
   */
  // eslint-disable-next-line
  public clean(input: Buffer): Buffer { return Buffer.from(''); };
}

/**
 * Clean HTML fragments based on the given options
 *
 * Prefer this over the `clean` function when using the same options many times
 */
// eslint-disable-next-line prefer-destructuring
export const Cleaner: typeof CleanerType = addon.Cleaner;

/**
 * Sanitizes an HTML fragment in a string according to the given options.
 * Shortcut for `(new Cleaner(options)).clean(input)`
 *
 * Use the `Cleaner` class instead when using the same options many times
 */
// eslint-disable-next-line prefer-destructuring
export const clean: (input: Buffer, options: Partial<BuilderOptions>) => Buffer = addon.clean;
