const { readFileSync } = require('fs');
const assert = require('assert');

const tests = {
  'should pass through simple well-formed whitelisted markup': [
    '<div><p>Hello <b>there</b></p></div>',
    '<div><p>Hello <b>there</b></p></div>'
  ],
  'should respect text nodes at top level': [
    'Blah blah blah<p>Whee!</p>', 'Blah blah blah<p>Whee!</p>'
  ],
  'should reject markup not whitelisted without destroying its text': [
    '<div><wiggly>Hello</wiggly></div>', '<div>Hello</div>'
  ],
  'should reject attributes not whitelisted': [
    '<a href="foo.html" whizbang="whangle">foo</a>', '<a href="foo.html">foo</a>'
  ],
  'should reject hrefs that are not relative, ftp, http, https or mailto': [
    '<a href="http://google.com">google</a><a href="https://google.com">https google</a><a href="ftp://example.com">ftp</a><a href="mailto:test@test.com">mailto</a><a href="/relative.html">relative</a><a href="javascript:alert(0)">javascript</a>',
    '<a href="http://google.com">google</a><a href="https://google.com">https google</a><a href="ftp://example.com">ftp</a><a href="mailto:test@test.com">mailto</a><a href="/relative.html">relative</a><a>javascript</a>'
  ],
  'should cope identically with capitalized attributes and tags and should tolerate capitalized schemes': [
    '<A HREF="http://google.com">google</a><a href="HTTPS://google.com">https google</a><a href="ftp://example.com">ftp</a><a href="mailto:test@test.com">mailto</a><a href="/relative.html">relative</a><a href="javascript:alert(0)">javascript</a>',
    '<a href="http://google.com">google</a><a href="HTTPS://google.com">https google</a><a href="ftp://example.com">ftp</a><a href="mailto:test@test.com">mailto</a><a href="/relative.html">relative</a><a>javascript</a>'
  ],
  'should drop the content of script elements': [
    '<script>alert("ruhroh!");</script><p>Paragraph</p>', '<p>Paragraph</p>'
  ],
  'should drop the content of style elements': [
    '<style>.foo { color: blue; }</style><p>Paragraph</p>', '<p>Paragraph</p>'
  ],
  'should drop the content of textarea elements': [
    '<textarea>Nifty</textarea><p>Paragraph</p>', '<p>Paragraph</p>'
  ],
  'should drop the content of textarea elements but keep the closing parent tag, when nested': [
    '<p>Paragraph<textarea>Nifty</textarea></p>', '<p>Paragraph</p>'
  ],
  'should retain the content of fibble elements by default': [
    '<fibble>Nifty</fibble><p>Paragraph</p>', 'Nifty<p>Paragraph</p>'
  ],
  'should preserve entities as such': [
    '<a name="&lt;silly&gt;">&lt;Kapow!&gt;</a>', '<a name="&lt;silly&gt;">&lt;Kapow!&gt;</a>'
  ],
  'should dump comments': [
    '<p><!-- Blah blah -->Whee</p>', '<p>Whee</p>'
  ],
  'should dump a sneaky encoded javascript url': [
    '<a href="&#106;&#97;&#118;&#97;&#115;&#99;&#114;&#105;&#112;&#116;&#58;&#97;&#108;&#101;&#114;&#116;&#40;&#39;&#88;&#83;&#83;&#39;&#41;">Hax</a>', '<a>Hax</a>'
  ],
  'should dump an uppercase javascript url': [
    '<a href="JAVASCRIPT:alert(\'foo\')">Hax</a>', '<a>Hax</a>'
  ],
  'should dump a javascript URL with a comment in the middle (probably only respected by browsers in XML data islands, but just in case someone enables those)': [
    '<a href="java<!-- -->script:alert(\'foo\')">Hax</a>', '<a>Hax</a>'
  ],
  'should not mess up a hashcode with a : in it': [
    '<a href="awesome.html#this:stuff">Hi</a>', '<a href="awesome.html#this:stuff">Hi</a>'
  ],
  'should dump character codes 1-32 before testing scheme': [
    '<a href="java\0&#14;\t\r\n script:alert(\'foo\')">Hax</a>', '<a>Hax</a>'
  ],
  'should still like nice schemes': [
    '<a href="http://google.com/">Hi</a>', '<a href="http://google.com/">Hi</a>'
  ],
  'should still like nice relative URLs': [
    '<a href="hello.html">Hi</a>', '<a href="hello.html">Hi</a>'
  ],
  'ipsum': [
    readFileSync(`${__dirname}/ipsum.html`, 'utf8')
  ]
};

const { Ammonia } = require('../lib');
const ammonia = new Ammonia();

const createDOMPurify = require('dompurify');
const { JSDOM } = require('jsdom');
 
const window = (new JSDOM('')).window;
const DOMPurify = createDOMPurify(window);

const xss = require('xss');

const sanitizeHtml = require('sanitize-html');


const trials = 10000;
function bench(name, cleanFn, buffers) {
  const sources = buffers ? Object.values(tests).map(([dirty]) => Buffer.from(dirty)) : Object.values(tests).map(([dirty]) => dirty);

  let totalNs = 0;
  for (let i = 0; i < trials; i += 1) {
    sources.forEach(dirty => {
      const before = process.hrtime();
      
      const clean = cleanFn(dirty);
      
      const [, ns] = process.hrtime(before);
      totalNs += ns;

      assert(clean);
    });
  }

  const meanNs = totalNs / trials;
  console.log(`${name.padStart(20)} took ${meanNs.toFixed().padStart(7)}ns on average`);
}

console.log('\n Running benchmarks...');

bench('ammonia + Buffer', input => ammonia.clean(input), true);
bench('ammonia', input => ammonia.sanitize(input));
bench('DOMPurify', input => DOMPurify.sanitize(input));
bench('xss', input => xss(input));
bench('sanitize-html', input => sanitizeHtml(input));

console.log('\n');
