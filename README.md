**_You should probably just use sanitize-html instead!_**

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

Not Really Todo
-----
- Add a compatibility layer to make ammonia-node a drop-in replacement for [sanitize-html]

Benchmarks
-----

With some [very naive benchmarks](tests/benchmark.js), it looks like ammonia-node is a little faster than sanitize-html.

| tool | time (less is better) |
|----------|---------|
| [DOMPurify] on JSDOM | 7565319ns |
| [sanitize-html]      | 677818ns  |
| ammonia w/ string    | 499031ns  |
| ammonia w/ Buffer    | 474540ns  |
| [xss]                | 219687ns  |

[sanitize-html]: https://github.com/punkave/sanitize-html "provides a simple HTML sanitizer with a clear API"
[xss]: https://github.com/leizongmin/js-xss "Sanitize untrusted HTML (to prevent XSS) with a configuration specified by a Whitelist"
[DOMPurify]: https://github.com/cure53/DOMPurify "DOMPurify - a DOM-only, super-fast, uber-tolerant XSS sanitizer"

Thanks
------

Thanks to the [awesome people behind Ammonia] for providing a fast HTML sanitizer library in rust.

[awesome people behind Ammonia]: https://github.com/rust-ammonia/ammonia/graphs/contributors "Ammonia Contributors"
