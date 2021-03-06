extern crate getopts;
extern crate typedopts;
extern crate serialize;

use getopts::{reqopt,getopts};
use std::os;
use typedopts::{DecoderResult,UnimplementedDecoder,MissingField,ExpectedType};

#[deriving(Decodable)]
struct Args {
  name:     ~str,
  quantity: uint
}

fn main() {
  let args = os::args();
  let opts = ~[
    reqopt("n", "name", "insert a name here", ""),
    reqopt("q", "quantity", "insert a quantity here", "")
  ];

  let matches = match getopts(args.tail(), opts) {
    Ok(m) => { m },
    Err(f) => { println!("{}", f.to_err_msg()); return; }
  };

  let result: DecoderResult<Args> = typedopts::decode(matches);
  match result {
    Ok(decoded) => {
      println!("name is {}", decoded.name);
      println!("quantity is {}", decoded.quantity);
    },
    Err(f) => { println!("{}", f.to_err_msg()); return; }
  }
}
