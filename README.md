DML is short for Dan's Markup Language  
This project is a DML to HTML converter written in Rust.

A sample DML document is in `ex.dml`

Usage of this program is as follows:  
```
$ cargo build --release
$ cd target/release
$ cat source.dml | sitegen > out.html
```

# license  
This is under the MIT license.
