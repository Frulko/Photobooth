

// use photobooth::my_mod;
use photobooth::camera::Camera;
use photobooth::camera;
use photobooth;
// mod camera;

fn function() {
    println!("called `function()`");
}

fn main() {
    function();
    // my_mod::public_function();
    // my_mod::indirect_function();

    // photobooth::test();
    let camera_instance = Camera::new();
    camera::gphoto::init();

    let p = photobooth::Image::new(1003);

    println!("1: {:?}", p.image_bytes);
    println!("2: {:?}", camera_instance.is_connected());

    // my_mod::public_function();

    camera::hosting();
    // camera::gphoto::init();
}
