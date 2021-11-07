use std::process::Command;

// maybe need a dedicated library ?

pub struct Printer {
  usb_id: String,
  max_paper: i32,
  min_paper: i32,
}

impl Printer{

  pub fn new() -> Printer {
    // all these settings need to be in a config.yml along with the query printer command
    let mut printer = Printer {
      usb_id: "3125:129D",
      max_paper: 700
      min_paper: 10,
    };


    return tl;
  }

  fn get_counter() {
    Command::new("./dnp40").args(["-s"])
  }

  fn set_counter() {
    Command::new("./dnp40").args(["-p", "12"])
  }

  fn get_queue() {
    // from CUPS api or rust package ?
  }

  fn print() {
    Command::new("lp -s").args(["./file.jpg"])
  }
}