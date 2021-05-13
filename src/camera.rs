pub mod gphoto;
mod hosting;

pub use hosting::hosting;


pub struct Camera {
  is_connected: bool,
}

impl Camera{

  pub fn new() -> Camera {
    let mut c = Camera {
      is_connected: false,
    };

    c.connect();

    return c;
  }

  fn connect(&mut self) {
    self.is_connected = true;
  }

  pub fn is_connected(&self) -> bool{
    self.is_connected
  }
}