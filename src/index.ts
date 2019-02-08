import * as assert from 'assert';
import { BuilderOptions, Cleaner, clean } from './native';

/* eslint-disable @typescript-eslint/camelcase */

export const defaults: BuilderOptions = {
  tags: [
    'a', 'abbr', 'acronym', 'area', 'article', 'aside', 'b', 'bdi',
    'bdo', 'blockquote', 'br', 'caption', 'center', 'cite', 'code',
    'col', 'colgroup', 'data', 'dd', 'del', 'details', 'dfn', 'div',
    'dl', 'dt', 'em', 'figcaption', 'figure', 'footer', 'h1', 'h2',
    'h3', 'h4', 'h5', 'h6', 'header', 'hgroup', 'hr', 'i', 'img',
    'ins', 'kbd', 'kbd', 'li', 'map', 'mark', 'nav', 'ol', 'p', 'pre',
    'q', 'rp', 'rt', 'rtc', 'ruby', 's', 'samp', 'small', 'span',
    'strike', 'strong', 'sub', 'summary', 'sup', 'table', 'tbody',
    'td', 'th', 'thead', 'time', 'tr', 'tt', 'u', 'ul', 'var', 'wbr',
  ],
  clean_content_tags: [],
  tag_attributes: {
    a: ['href', 'hreflang'],
    bdo: ['dir'],
    blockquote: ['cite'],
    col: ['align', 'char', 'charoff', 'span'],
    colgroup: ['align', 'char', 'charoff', 'span'],
    del: ['cite', 'datetime'],
    hr: ['align', 'size', 'width'],
    img: ['align', 'alt', 'height', 'src', 'width'],
    ins: ['cite', 'datetime'],
    ol: ['start'],
    q: ['cite'],
    table: ['align', 'char', 'charoff', 'summary'],
    tbody: ['align', 'char', 'charoff'],
    td: ['align', 'char', 'charoff', 'colspan', 'headers', 'rowspan'],
    tfoot: ['align', 'char', 'charoff'],
    th: ['align', 'char', 'charoff', 'colspan', 'headers', 'rowspan', 'scope'],
    thead: ['align', 'char', 'charoff'],
    tr: ['align', 'char', 'charoff'],
  },
  tag_attribute_values: {},
  generic_attributes: ['lang', 'title'],
  url_schemes: [
    'bitcoin', 'ftp', 'ftps', 'geo', 'http', 'https', 'im', 'irc',
    'ircs', 'magnet', 'mailto', 'mms', 'mx', 'news', 'nntp',
    'openpgp4fpr', 'sip', 'sms', 'smsto', 'ssh', 'tel', 'url',
    'webcal', 'wtai', 'xmpp',
  ],
  url_relative: 'pass-through',
  link_rel: 'noopener noreferrer',
  allowed_classes: {},
  strip_comments: true,
  id_prefix: null,
};

function validateOptions(options: Partial<BuilderOptions>): void {
  const hasOwnProperty = <T>(
    obj: T,
    key: keyof T
  ): boolean => Object.prototype.hasOwnProperty.call(obj, key);

  function assertArray(option: keyof BuilderOptions): void {
    if (!hasOwnProperty(options, option)) { return; }

    const arr = options[option];
    assert(
      Array.isArray(arr) && arr.every(elem => typeof elem === 'string'),
      `Option '${option}' must be an array of strings`
    );
  }
  function assertMapArray(option: keyof BuilderOptions): void {
    if (!hasOwnProperty(options, option)) { return; }

    const error = `Option '${option}' must be an object mapping tags to arrays of strings`;

    const map = options[option] as {
      [tag: string]: string[];
    };

    assert(typeof map === 'object', error);

    Object.keys(map).forEach((key) => {
      const arr = map[key];
      assert(Array.isArray(arr) && arr.every(elem => typeof elem === 'string'), error);
    });
  }
  function assertMapMapArray(option: keyof BuilderOptions): void {
    if (!hasOwnProperty(options, option)) { return; }

    const error = `Option '${option}' must be an object mapping tags to objects mapping attributes to arrays of strings`;

    const map = options[option] as {
      [tag: string]: {
        [attr: string]: string[];
      };
    };

    assert(typeof map === 'object', error);

    Object.keys(map).forEach((mapKey) => {
      const subMap = map[mapKey];

      assert(typeof subMap === 'object', error);

      Object.keys(subMap).forEach((key) => {
        const arr = subMap[key];
        assert(
          Array.isArray(arr) && arr.every(elem => typeof elem === 'string'),
          error
        );
      });
    });
  }
  function assertDisjoint(a: string[] | undefined, b: string[] | undefined, message: string): void {
    if (!a || !b) { return; }

    const s = new Set(a);
    assert(b.every(val => !s.has(val)), message);
  }

  assertArray('tags');
  assertArray('clean_content_tags');
  assertDisjoint(options.tags || defaults.tags, options.clean_content_tags,
    'Option \'clean_content_tags\' must not share any tags with \'tags\'');
  assertDisjoint(Object.keys(options.tag_attributes || defaults.tag_attributes),
    options.clean_content_tags,
    'Option \'clean_content_tags\' must not share any tags with \'tag_attributes\'');
  assertMapArray('tag_attributes');
  assertMapMapArray('tag_attribute_values');
  assertArray('generic_attributes');
  assertArray('url_schemes');
  if (hasOwnProperty(options, 'url_relative')) {
    assert(
      options.url_relative === 'deny' ||
      options.url_relative === 'pass-through' ||
      (
        Array.isArray(options.url_relative) &&
        options.url_relative[0] === 'resolve-with-base' &&
        typeof options.url_relative[1] === 'string'
      ),
      'Option \'url_relative\' must be \'deny\', \'pass-through\', or a tuple [\'resolve-with-base\', string]'
    );
  }
  if (hasOwnProperty(options, 'link_rel')) {
    assert(options.link_rel === null || typeof options.link_rel === 'string',
      'Option \'link_rel\' must be \'null\' or a string');
  }
  assertMapArray('allowed_classes');
  assert(
    Object.keys(options.allowed_classes || defaults.allowed_classes).length === 0 ||
    !(options.generic_attributes || defaults.generic_attributes).includes('class'),
    'Option \'generic_attributes\' can not contain \'class\' if there are \'allowed_classes\''
  );
  assert(
    Object.keys(options.allowed_classes || defaults.allowed_classes).every(
      tag => !(options.tag_attributes || defaults.tag_attributes)[tag].includes('class')
    ),
    'Option \'tag_attributes\' can not contain \'class\' for any tags in \'allowed_classes\''
  );
  if (hasOwnProperty(options, 'strip_comments')) {
    assert(typeof options.strip_comments === 'boolean');
  }
  if (hasOwnProperty(options, 'id_prefix')) {
    assert(options.id_prefix === null || typeof options.id_prefix === 'string',
      'Option \'id_prefix\' must be \'null\' or a string');
  }
}

type Options = Partial<BuilderOptions & { validate: boolean }>;

export class Ammonia extends Cleaner {
  // Configure options
  public constructor(options?: Options) {
    const opts = options || {};
    if (options.validate) { validateOptions(opts); }
    super(opts);
  }

  /**
   * Sanitizes an HTML fragment in a string according to the configured options.
   */
  public sanitize(input: string): string {
    return super.clean(input);
  }
}

/**
 * Sanitizes an HTML fragment in a string according to the given options.
 */
export function sanitize(input: string, options?: Options): string {
  const opts = options || {};
  if (options.validate) { validateOptions(opts); }
  return clean(input, opts);
}
