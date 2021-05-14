
use std::collections::HashMap;
pub mod camera;
pub mod websocket;

mod image;


pub use camera::Camera;
pub use image::Image;



pub fn test() {
  let mut map = HashMap::new();
  map.insert(1, 2);

  println!(">> {:?}", map);
}

pub mod my_mod {
  fn private_function() {
      println!("my_mod.private_function");
  }
  pub fn public_function() {
      println!("my_mod.public_function");
  }
  pub fn indirect_function() {
      println!("my_mod.indirect_function");
      self::private_function();
  }
}
