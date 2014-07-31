/*!
A demo partial wrapper of the smtplib library from python to demonstrate rustpy.

```rust
#[link(name="python2.7")]
extern crate rustpy;
extern crate pysmtplib;

fn main() {
  let py = rustpy::PyState::new();
  let state = PyState::new();
  let mut session = pysmtplib::SMTP::new(&state, "smtp.gmail.com", 587);
  session.ehlo();
  session.starttls();
  let username = "example@example.com";
  let _ = session.login(username, "example").unwrap();
  let _ = session.sendmail(username, username, "Hello world from rust").unwrap();
}
```
*/

#![crate_type = "lib"]
#![feature(macro_rules)]

#[link(name="python2.7")]
extern crate rustpy;
extern crate debug;

use rustpy::{PyType, PyState, PyObject, PyError};

pub struct SMTP<'a> {
  state : &'a PyState,
  py_object : PyObject<'a>
}

impl<'a> SMTP<'a> {
  pub fn new(state : &'a PyState, server : &str, port : uint) -> SMTP<'a> {
    let smtplib = state.get_module("smtplib").unwrap();
    let args = (server, port).to_py_object(state).unwrap();
    let smtp = smtplib.get_func("SMTP").unwrap();
    let py_object = smtp.call(&args).unwrap();
    SMTP {state : state, py_object : py_object}
  }

  pub fn ehlo(&mut self) {
    let args = PyObject::empty_tuple(self.state);
    let _ = self.py_object.get_func("ehlo").unwrap().call(&args).unwrap();
  }

  pub fn starttls(&mut self) {
    let args = PyObject::empty_tuple(self.state);
    let _ = self.py_object.get_func("starttls").unwrap().call(&args).unwrap();
  }

  pub fn login(&mut self, username : &str, password : &str) -> Result<(), PyError>{
    let args = (username, password).to_py_object(self.state).unwrap();
    let res = self.py_object.get_func("login").unwrap().call(&args);
    res.map(|_| ())
  }

  pub fn sendmail(&mut self, username : &str, recipient : &str, content : &str) -> Result<(), PyError> {
    let args = (username, recipient, content).to_py_object(self.state).unwrap();
    let res = self.py_object.get_func("sendmail").unwrap().call(&args);
    res.map(|_| ())
  }
}
