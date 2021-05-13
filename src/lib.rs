
use std::collections::HashMap;
pub mod camera;
mod image;

pub use camera::Camera;
pub use image::Image;





// pub struct Image {

// }

// impl Image {
//   pub fn new() -> image::Image {
//     image::Image{
//       image_bytes: 73,
//       printable_image: 42
//     }
//   }
// }

// pub fn plop() -> image::Image{
//   image::Image{
//     image_bytes: 75,
//     printable_image: 42
//   }
// }


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
