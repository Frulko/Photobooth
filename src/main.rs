
use photobooth::Camera;
use photobooth::camera;
use photobooth::websocket;
use photobooth::websocket::{WebSocketData};

extern crate gphoto;

fn main() {
    
    let mut context = gphoto::Context::new().unwrap();

    match gphoto::Camera::autodetect(&mut context){
        Ok(c) => {
            let mut camera = c;
            let summary = camera.summary(&mut context).unwrap();


            // let capture = camera.capture_image(&mut context).unwrap();
            // let mut file = gphoto::FileMedia::create(Path::new(&*capture.basename())).unwrap();
        
            // camera.download(&mut context, &capture, &mut file).unwrap();
            println!("Camera connected !");
            // println!("{}", summary);

            for storage in camera.storage(&mut context).unwrap() {
                println!("       base dir = {:?}", storage.base_dir());
                println!("          label = {:?}", storage.label());
                println!("    description = {:?}", storage.description());
                println!("   storage type = {:?}", storage.storage_type());
                println!("filesystem type = {:?}", storage.filesystem_type());
                println!("    access type = {:?}", storage.access_type());
                println!("    capacity kb = {:?}", storage.capacity_kbytes());
                println!("        free kb = {:?}", storage.free_kbytes());
                println!("    free images = {:?}", storage.free_images());
            }
        },
        Err(err) => {
            if err.kind() == gphoto::ErrorKind::ModelNotFound {
                println!("Camera not found");
            } else {
                println!("There is an error, {:?}", err);
            }
        }
    };
  
    
    let camera_instance = Camera::new();
    camera::gphoto::init();

    let p = photobooth::Image::new(1003);

    println!("1: {:?}", p.image_bytes);
    println!("2: {:?}", camera_instance.is_connected());

    websocket::start(3065, handle_websocket);
}


fn handle_websocket(data: websocket::WebSocketData) {

    match data {
        WebSocketData::JSON(_) => println!("JSON way !"),
        WebSocketData::Text(_) => println!("Old way !")
    }

    // println!("Receive JSON data here {:?}", data["age"]);
}