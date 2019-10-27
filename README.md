# the-super-tiny-compiler-in-rust

I have never written my own compiler. I want to change that. I have a basic
understanding of how compilers work but feel like I might be missing parts of
the picture.

Maybe this is fun?

* **Based on https://github.com/jamiebuilds/the-super-tiny-compiler but implemented in Rust.**
* This has nothing to do with [rustc](https://github.com/rust-lang/rust)

I consider this done when I've got the following working:

```
 *                  LISP                      C
 *
 *   2 + 2          (add 2 2)                 add(2, 2)
 *   4 - 2          (subtract 4 2)            subtract(4, 2)
 *   2 + (4 - 2)    (add 2 (subtract 4 2))    add(2, subtract(4, 2))
```

There will still be lots of opportunities for refactoring in the end.
