# LR

[![Build Status](https://travis-ci.org/adjivas/lr.svg?branch=master)](https://travis-ci.org/adjivas/lr)

#### Current output
```shell
$ cargo run
   Compiling example v0.1.0 (file:///home/adjivas/repos/lr_rendu)
error: failed to run custom build command for `example v0.1.0 (file:///home/adjivas/repos/lr_rendu)`
process didn't exit successfully: `/home/adjivas/repos/lr_rendu/target/debug/build/example-cb098efc7a4b65b9/build-script-build` (exit code: 1)
--- stdout
processing file `/home/adjivas/repos/lr_rendu/src/lr.lalrpop`
/home/adjivas/repos/lr_rendu/src/lr.lalrpop:22:5: 22:17 error: ambiguity detected between the terminal `r#"[:alnum:]+"#` and the terminal `r#"[0-9]+"#`

--- stderr
      r"[:alnum:]+" => util::Screen::new(String::from_str(<>).unwrap().into_bytes() ),
      ~~~~~~~~~~~~~
```

#### Knowledge
* https://dfockler.github.io/2016/09/15/lalrpop.html
* http://smallcultfollowing.com/babysteps/blog/2015/09/14/lalrpop/?utm_content=buffer39dce&utm_medium=social&utm_source=twitter.com&utm_campaign=buffer
* http://notes.willcrichton.net/parsing-strategies-in-rust
* https://doc.rust-lang.org/regex/regex/index.html
