'use strict';

// test validateOptions

const assert = require('assert');
const { defaults, sanitize } = require('../lib');

const clean = (input, options) => sanitize(input, { ...(options || {}), validate: true });

describe('validator', function () {

  describe('should fail', function () {
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
      }, {
        message: 'Option \'tag_attributes\' can not contain \'class\' for any tags in \'allowed_classes\''
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
      }, {
        message: 'Option \'generic_attributes\' can not contain \'class\' if there are \'allowed_classes\''
      });
    });
    it('on clean content tag attribute', function () {
      assert.throws(function () {
        clean("", {
          tags: defaults.tags.filter(x => x !== "a"),
          clean_content_tags: ["a"],
        });
      }, {
        message: 'Option \'clean_content_tags\' must not share any tags with \'tag_attributes\''
      });
    });
    it('on clean content tag', function () {
      assert.throws(function () {
        clean("", {
          clean_content_tags: ["a"],
        });
      }, {
        message: 'Option \'clean_content_tags\' must not share any tags with \'tags\''
      });
    });
  });

});
