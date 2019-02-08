# ammonia-node

Node bindings for the [Ammonia HTML sanitation library]

Ammonia is a whitelist-based HTML sanitization library. It is designed to
prevent cross-site scripting, layout breaking, and clickjacking caused
by untrusted user-provided HTML being mixed into a larger web page.

Ammonia uses [html5ever] to parse and serialize document fragments the same way browsers do,
so it is extremely resilient to syntactic obfuscation.

Ammonia parses its input exactly according to the HTML5 specification;
it will not linkify bare URLs, insert line or paragraph breaks, or convert `(C)` into &copy;.
If you want that, use a markup processor before running the sanitizer.

[html5ever]: https://github.com/servo/html5ever "The HTML parser in Servo"
[Ammonia HTML sanitation library]: https://github.com/rust-ammonia/ammonia "Repair and secure untrusted HTML"

Installation
-----------

To use `ammonia-node`, add it to your project via `npm` or `yarn`

```
npm install ammonia
yarn add ammonia
```

Thanks
------

Thanks to the [awesome people behind Ammonia] for providing a fast HTML sanitizer library in rust.

[awesome people behind Ammonia]: https://github.com/rust-ammonia/ammonia/graphs/contributors "Ammonia Contributors"
