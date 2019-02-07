'use strict';

const assert = require('assert');
const native = require('../native');
const clean = (input) => native.clean(input, {});

describe('native', function () {

  describe('default clean', function () {
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
});
