pysmtplib.rs
============

Rust wrapper around part of the python smtplib to show how rustpy works.

```rust
#[link(name="python2.7")]
extern crate rustpy;
extern crate pysmtplib;
use pysmtplib::SMTP;
use rustpy::PyState;

fn main() {
  let state = PyState::new();
  let mut session = SMTP::new(&state, "smtp.gmail.com", 587);
  session.ehlo();
  session.starttls();
  let username = "example@gmail.com";
  let _ = session.login(username, "example").unwrap();
  let _ = session.sendmail(username, username, "Hello world from rust").unwrap();
}
```
