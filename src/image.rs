

pub struct Image{
  pub image_bytes: i32,
  pub printable_image: i32
  // backgroundImage: bool
}

impl Image{
  pub fn new(i: i32) -> Image {
    Image{
      image_bytes: i,
      printable_image: 42
    }
  }
}