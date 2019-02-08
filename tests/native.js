'use strict';

// test the native module
// most of the tests are adapted from the ammonia repository
// https://github.com/rust-ammonia/ammonia/blob/master/src/lib.rs

const assert = require('assert');
const native = require('../lib/native');
const { defaults } = require('../lib');

describe('native', function () {

  describe('default cleaner instance', function () {
    const cleaner = new native.Cleaner({});
    const clean = input => cleaner.clean(input);

    it('included angles', function () {
      let fragment = "1 < 2";
      let result = clean(fragment);
      assert.equal(result, "1 &lt; 2");
    });

    it('remove script', function () {
      let fragment = "an <script>evil()</script> example";
      let result = clean(fragment);
      assert.equal(result, "an  example");
    });

    it('ignore link', function () {
      let fragment = "a <a href=\"http://www.google.com\">good</a> example";
      let expected = "a <a href=\"http://www.google.com\" rel=\"noopener noreferrer\">good</a> example";
      let result = clean(fragment);
      assert.equal(result, expected);
    });
    it('remove unsafe link', function () {
      let fragment = "an <a onclick=\"evil()\" href=\"http://www.google.com\">evil</a> example";
      let result = clean(fragment);
      assert.equal(
        result,
        "an <a href=\"http://www.google.com\" rel=\"noopener noreferrer\">evil</a> example"
      );
    });
    it('remove js link', function () {
      let fragment = "an <a href=\"javascript:evil()\">evil</a> example";
      let result = clean(fragment);
      assert.equal(result, "an <a rel=\"noopener noreferrer\">evil</a> example");
    });
    it('tag rebalance', function () {
      let fragment = "<b>AWESOME!";
      let result = clean(fragment);
      assert.equal(result, "<b>AWESOME!</b>");
    });
  });

  const clean = (input, options) => native.clean(input, options || {});

  describe('options', function () {
    it('allow url relative', function () {
      let fragment = "<a href=test>Test</a>";
      let result = clean(fragment, {
        url_relative: 'pass-through',
      });

      assert.equal(
        result,
        "<a href=\"test\" rel=\"noopener noreferrer\">Test</a>"
      );
    });
    it('rewrite url relative', function () {
      let fragment = "<a href=test>Test</a>";
      let result = clean(fragment, {
        url_relative: ['resolve-with-base', "http://example.com/"],
      });
      assert.equal(
        result,
        "<a href=\"http://example.com/test\" rel=\"noopener noreferrer\">Test</a>"
      );
    });
    // it('attribute filter nop', function () {
    //     let fragment = "<a href=test>Test</a>";
    //     let result = Builder::new()
    //         .attribute_filter(|elem, attr, value| {
    //             assert.equal("a", elem);
    //             assert!(match (attr, value) {
    //                 ("href", "test") => true,
    //                 ("rel", "noopener noreferrer") => true,
    //                 _ => false,
    //             }, value.to_string());
    //             Some(value.into())
    //         })
    //         .clean(fragment)
    //         .to_string();
    //     assert.equal(
    //         result,
    //         "<a href=\"test\" rel=\"noopener noreferrer\">Test</a>"
    //     );
    // });
    //
    // it('attribute filter drop', function () {
    //     let fragment = "Test<img alt=test src=imgtest>";
    //     let result = Builder::new()
    //         .attribute_filter(|elem, attr, value| {
    //             assert.equal("img", elem);
    //             match (attr, value) {
    //                 ("src", "imgtest") => None,
    //                 ("alt", "test") => Some(value.into()),
    //                 _ => panic!("unexpected"),
    //             }
    //         })
    //         .clean(fragment)
    //         .to_string();
    //     assert.equal(
    //         result,
    //         r#"Test<img alt="test">"#
    //     );
    // });
    //
    // it('url filter absolute', function () {
    //     let fragment = "Test<img alt=test src=imgtest>";
    //     let result = Builder::new()
    //         .attribute_filter(|elem, attr, value| {
    //             assert.equal("img", elem);
    //             match (attr, value) {
    //                 ("src", "imgtest") => Some(format!("https://example.com/images/{}", value).into()),
    //                 ("alt", "test") => None,
    //                 _ => panic!("unexpected"),
    //             }
    //         })
    //         .url_relative(UrlRelative::RewriteWithBase(Url::parse("http://wrong.invalid/").unwrap()))
    //         .clean(fragment)
    //         .to_string();
    //     assert.equal(
    //         result,
    //         r#"Test<img src="https://example.com/images/imgtest">"#
    //     );
    // });
    //
    // it('url filter relative', function () {
    //     let fragment = "Test<img alt=test src=imgtest>";
    //     let result = Builder::new()
    //         .attribute_filter(|elem, attr, value| {
    //             assert.equal("img", elem);
    //             match (attr, value) {
    //                 ("src", "imgtest") => Some("rewrite".into()),
    //                 ("alt", "test") => Some("altalt".into()),
    //                 _ => panic!("unexpected"),
    //             }
    //         })
    //         .url_relative(UrlRelative::RewriteWithBase(Url::parse("https://example.com/base/#").unwrap()))
    //         .clean(fragment)
    //         .to_string();
    //     assert.equal(
    //         result,
    //         r#"Test<img alt="altalt" src="https://example.com/base/rewrite">"#
    //     );
    // });

    it('rewrite url relative no rel', function () {
      let fragment = "<a href=test>Test</a>";
      let result = clean(fragment, {
        url_relative: ['resolve-with-base', "http://example.com/"],
        link_rel: null,
      });
      assert.equal(result, "<a href=\"http://example.com/test\">Test</a>");
    });
    it('deny url relative', function () {
      let fragment = "<a href=test>Test</a>";
      let result = clean(fragment, {
        url_relative: 'deny',
      });
      assert.equal(result, "<a rel=\"noopener noreferrer\">Test</a>");
    });
    it('replace rel', function () {
      let fragment = "<a href=test rel=\"garbage\">Test</a>";
      let result = clean(fragment, {
        url_relative: 'pass-through',
      });
      assert.equal(
        result,
        "<a href=\"test\" rel=\"noopener noreferrer\">Test</a>"
      );
    });
    it('consider rel still banned', function () {
      let fragment = "<a href=test rel=\"garbage\">Test</a>";
      let result = clean(fragment, {
        url_relative: 'pass-through',
        link_rel: null,
      });
      assert.equal(result, "<a href=\"test\">Test</a>");
    });
    it('object data', function () {
      let fragment = "<span data=\"javascript:evil()\">Test</span><object data=\"javascript:evil()\"></object>M";
      let expected = "<span data=\"javascript:evil()\">Test</span><object></object>M";
      let result = clean(fragment, {
        tags: ["span", "object"],
        generic_attributes: ["data"],
      });
      assert.equal(result, expected);
    });
    it('remove attributes', function () {
      let fragment = "<table border=\"1\"><tr></tr></table>";
      let result = clean(fragment);
      assert.equal(
        result,
        "<table><tbody><tr></tr></tbody></table>"
      );
    });
    it('quotes in attrs', function () {
      let fragment = "<b title='\"'>contents</b>";
      let result = clean(fragment);
      assert.equal(result, "<b title=\"&quot;\">contents</b>");
    });
    // #[test]
    // #[should_panic]
    // fn panic_if_rel_is_allowed_and_replaced_generic() {
    //     Builder::new()
    //         .link_rel(Some("noopener noreferrer"))
    //         .generic_attributes(hashset!["rel"])
    //         .clean("something");
    // }
    // #[test]
    // #[should_panic]
    // fn panic_if_rel_is_allowed_and_replaced_a() {
    //     Builder::new()
    //         .link_rel(Some("noopener noreferrer"))
    //         .tag_attributes(hashmap![
    //             "a" => hashset!["rel"],
    //         ])
    //         .clean("something");
    // });
    // it('no panic if rel is allowed and replaced span', function () {
    //     Builder::new()
    //         .link_rel(Some("noopener noreferrer"))
    //         .tag_attributes(hashmap![
    //             "span" => hashset!["rel"],
    //         ])
    //         .clean("<span rel=\"what\">s</span>");
    // });
    // it('no panic if rel is allowed and not replaced generic', function () {
    //     Builder::new()
    //         .link_rel(None)
    //         .generic_attributes(hashset!["rel"])
    //         .clean("<a rel=\"what\">s</a>");
    // });
    // it('no panic if rel is allowed and not replaced a', function () {
    //     Builder::new()
    //         .link_rel(None)
    //         .tag_attributes(hashmap![
    //             "a" => hashset!["rel"],
    //         ])
    //         .clean("<a rel=\"what\">s</a>");
    // });
    it('dont close void elements', function () {
      let fragment = "<br>";
      let result = clean(fragment);
      assert.equal(result, "<br>");
    });
    it('remove non allowed classes', function () {
      let fragment = "<p class=\"foo bar\"><a class=\"baz bleh\">Hey</a></p>";
      let result = clean(fragment, {
        link_rel: null,
        allowed_classes: {
          "p": ["foo", "bar"],
          "a": ["baz"],
        },
      });
      assert.equal(
        result,
        "<p class=\"foo bar\"><a class=\"baz\">Hey</a></p>"
      );
    });
    it('remove non allowed classes with tag class', function () {
      let fragment = "<p class=\"foo bar\"><a class=\"baz bleh\">Hey</a></p>";
      let result = clean(fragment, {
        link_rel: null,
        tag_attributes: {
          "div": ["class"],
        },
        allowed_classes: {
          "p": ["foo", "bar"],
          "a": ["baz"],
        },
      });
      assert.equal(
        result,
        "<p class=\"foo bar\"><a class=\"baz\">Hey</a></p>"
      );
    });
    it('remove non allowed attributes with tag attribute values', function () {
      let fragment = "<p data-label=\"baz\" name=\"foo\"></p>";
      let result = clean(fragment, {
        tag_attribute_values: {
          "p": {
            "data-label": ["bar"],
          },
        },
        tag_attributes: {
          "p": ["name"],
        },
      });
      assert.equal(
        result,
        "<p name=\"foo\"></p>",
      );
    });
    it('keep allowed attributes with tag attribute values', function () {
      let fragment = "<p data-label=\"bar\" name=\"foo\"></p>";
      let result = clean(fragment, {
        tag_attribute_values: {
          "p": {
            "data-label": ["bar"],
          },
        },
        tag_attributes: {
          "p": ["name"],
        },
      });
      assert.equal(
        result,
        "<p data-label=\"bar\" name=\"foo\"></p>",
      );
    });
    it('tag attribute values case insensitive', function () {
      let fragment = "<input type=\"CHECKBOX\" name=\"foo\">";
      let result = clean(fragment, {
        tags: ["input"],
        tag_attribute_values: {
          "input": {
            "type": ["checkbox"],
          },
        },
        tag_attributes: {
          "input": ["name"],
        },
      });
      assert.equal(
        result,
        "<input type=\"CHECKBOX\" name=\"foo\">",
      );
    });
    it('remove entity link', function () {
      let fragment = "<a href=\"&#x6A&#x61&#x76&#x61&#x73&#x63&#x72&#x69&#x70&#x74&#x3A&#x61" +
                      "&#x6C&#x65&#x72&#x74&#x28&#x27&#x58&#x53&#x53&#x27&#x29\">Click me!</a>";
      let result = clean(fragment);
      assert.equal(
        result,
        "<a rel=\"noopener noreferrer\">Click me!</a>"
      );
    });
    // it('remove relative url evaluate', function () {
    //     fn is_absolute_path(url: &str) -> bool {
    //         let u = url.as_bytes();
    //         // `//a/b/c` is "protocol-relative", meaning "a" is a hostname
    //         // `/a/b/c` is an absolute path, and what we want to do stuff to.
    //         u.get(0) == Some(&b'/') && u.get(1) != Some(&b'/')
    //     }
    //     fn is_banned(url: &str) -> bool {
    //         let u = url.as_bytes();
    //         u.get(0) == Some(&b'b') && u.get(1) == Some(&b'a')
    //     }
    //     fn evaluate(url: &str) -> Option<Cow<str>> {
    //         if is_absolute_path(url) {
    //             Some(Cow::Owned(String::from("/root") + url))
    //         } else if is_banned(url) {
    //             None
    //         } else {
    //             Some(Cow::Borrowed(url))
    //         }
    //     }
    //     let a = Builder::new()
    //         .url_relative(UrlRelative::Custom(Box::new(evaluate)))
    //         .clean("<a href=banned>banned</a><a href=/test/path>fixed</a><a href=path>passed</a><a href=http://google.com/>skipped</a>")
    //         .to_string();
    //     assert.equal(a, "<a rel=\"noopener noreferrer\">banned</a><a href=\"/root/test/path\" rel=\"noopener noreferrer\">fixed</a><a href=\"path\" rel=\"noopener noreferrer\">passed</a><a href=\"http://google.com/\" rel=\"noopener noreferrer\">skipped</a>");
    // });
    // it('remove relative url evaluate b', function () {
    //     fn is_absolute_path(url: &str) -> bool {
    //         let u = url.as_bytes();
    //         // `//a/b/c` is "protocol-relative", meaning "a" is a hostname
    //         // `/a/b/c` is an absolute path, and what we want to do stuff to.
    //         u.get(0) == Some(&b'/') && u.get(1) != Some(&b'/')
    //     }
    //     fn is_banned(url: &str) -> bool {
    //         let u = url.as_bytes();
    //         u.get(0) == Some(&b'b') && u.get(1) == Some(&b'a')
    //     }
    //     fn evaluate(url: &str) -> Option<Cow<str>> {
    //         if is_absolute_path(url) {
    //             Some(Cow::Owned(String::from("/root") + url))
    //         } else if is_banned(url) {
    //             None
    //         } else {
    //             Some(Cow::Borrowed(url))
    //         }
    //     }
    //     let a = Builder::new()
    //         .url_relative(UrlRelative::Custom(Box::new(evaluate)))
    //         .clean("<a href=banned>banned</a><a href=banned title=test>banned</a><a title=test href=banned>banned</a>")
    //         .to_string();
    //     assert.equal(a, "<a rel=\"noopener noreferrer\">banned</a><a rel=\"noopener noreferrer\" title=\"test\">banned</a><a title=\"test\" rel=\"noopener noreferrer\">banned</a>");
    // });
    // it('remove relative url evaluate c', function () {
    //     // Don't run on absolute URLs.
    //     fn evaluate(_: &str) -> Option<Cow<str>> {
    //         return Some(Cow::Owned(String::from("invalid")));
    //     }
    //     let a = Builder::new()
    //         .url_relative(UrlRelative::Custom(Box::new(evaluate)))
    //         .clean("<a href=\"https://www.google.com/\">google</a>")
    //         .to_string();
    //     assert.equal(a, "<a href=\"https://www.google.com/\" rel=\"noopener noreferrer\">google</a>");
    // });
    it('clean children of bad element', function () {
      let fragment = "<bad><evil>a</evil>b</bad>";
      let result = clean(fragment);
      assert.equal(result, "ab");
    });
    it('reader input', function () {
      let fragment = "an <script>evil()</script> example";
      let result = clean(fragment);
      assert.equal(result, "an  example");
    });
    // it('reader_non_utf8', function () {
    //   let fragment = "non-utf8 \xF0\x90\x80string";
    //   let result = clean(fragment);
    //   assert.equal(result, "non-utf8 \u{fffd}string");
    // });
    it('display impl', function () {
      let fragment = "a <a>link</a>";
      let result = clean(fragment, {
        link_rel: null,
      });
      assert.equal(result, "a <a>link</a>");
    });
    it('id prefixed', function () {
      let fragment = "<a id=\"hello\"></a><b id=\"hello\"></a>";
      let result = clean(fragment, {
        tag_attributes: {
          "a": ["id"],
        },
        id_prefix: "prefix-",
      });
      assert.equal(result, "<a id=\"prefix-hello\" rel=\"noopener noreferrer\"></a><b></b>");
    });
    it('id already prefixed', function () {
      let fragment = "<a id=\"prefix-hello\"></a>";
      let result = clean(fragment, {
        tag_attributes: {
          "a": ["id"],
        },
        id_prefix: "prefix-",
      });
      assert.equal(result, "<a id=\"prefix-hello\" rel=\"noopener noreferrer\"></a>");
    });
    it('clean content tags', function () {
      let fragment = "<script type=\"text/javascript\"><a>Hello!</a></script>";
      let result = clean(fragment, {
        clean_content_tags: ["script"],
      });
      assert.equal(result, "");
    });
    it('only clean content tags', function () {
      let fragment = "<em>This is</em><script><a>Hello!</a></script><p>still here!</p>";
      let result = clean(fragment, {
        clean_content_tags: ["script"],
      });
      assert.equal(result, "<em>This is</em><p>still here!</p>");
    });
    it('clean removed default tag', function () {
      let fragment = "<em>This is</em><script><a>Hello!</a></script><p>still here!</p>";

      let tag_attributes = {
        ...defaults.tag_attributes,
        "a": defaults.tag_attributes.a
          .filter(x => !["href", "hreflang"].includes(x)),
      };
      let result = clean(fragment, {
        tags: defaults.tags.filter(x => x !== "a"),
        tag_attributes,
        clean_content_tags: ["script"],
      });
      assert.equal(result, "<em>This is</em><p>still here!</p>");
    });
  });

  describe('should panic', function () {
    it('on allowed classes tag attributes', function () {
      assert.throws(function () {
        let fragment = "<p class=\"foo bar\"><a class=\"baz bleh\">Hey</a></p>";
        clean(fragment, {
          link_rel: null,
          tag_attributes: {
            "p": ["class"],
            "a": ["class"],
          },
          allowed_classes: {
            "p": ["foo", "bar"],
            "a": ["baz"],
          },
        });
      });
    });
    it('on allowed classes generic attributes', function () {
      assert.throws(function () {
        let fragment = "<p class=\"foo bar\"><a class=\"baz bleh\">Hey</a></p>";
        clean(fragment, {
          link_rel: null,
          generic_attributes: ["class", "href", "some-foo"],
          allowed_classes: {
            "p": ["foo", "bar"],
            "a": ["baz"],
          },
        });
      });
    });
    it('on clean content tag attribute', function () {
      assert.throws(function () {
        clean("", {
          tags: defaults.tags.filter(x => x !== "a"),
          clean_content_tags: ["a"],
        });
      });
    });
    it('on clean content tag', function () {
      assert.throws(function () {
        clean("", {
          clean_content_tags: ["a"],
        });
      });
    });
  });

});
