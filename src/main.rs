use photobooth::Camera;
use photobooth::camera;
use photobooth::websocket;
use photobooth::websocket::{WebSocketData};
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use rouille::Request;
use rouille::Response;

extern crate gphoto;


static SOME_INT: i32 = 5;


fn main() {
    
    println!("Start: {}", SOME_INT);

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
  
    camera::gphoto::init();
    
    let camera_instance = Camera::new();
    let camera_mutex = Arc::new(Mutex::new(camera_instance));

    let p = photobooth::Image::new(1003);

    println!("1: {:?}", p.image_bytes);
    println!("2: {:?}", camera_mutex.lock().unwrap().is_connected());


    // let camera_mutex_clone = Arc::clone(&camera_mutex);
    // let handle = thread::spawn(move || {
    //     let mut camera_instance = camera_mutex_clone.lock().unwrap();

    //     println!("-1--: {:?}", camera_instance.is_connected());


      
    // });

      
    websocket::start("0.0.0.0", 3001, move |data| {
        match data {
            WebSocketData::JSON(data) => {
                // let mut data_mutex = camera_instance.lock().unwrap();
                // println!("2: {:?}", camera_instance.is_connected());
                println!("handle_websocket: {}", SOME_INT);
                println!("JSON way ! {:?}", data)
            },
            WebSocketData::Text(text) => println!("Old way ! {:?}", text)
        }
    });

    // println!("-ddd--: {:?}", camera_mutex.lock().unwrap().is_connected());
    // handle.join().unwrap();

   
    
    rouille::start_server("0.0.0.0:3003", move |request| {
        {
            // The `match_assets` function tries to find a file whose name corresponds to the URL
            // of the request. The second parameter (`"."`) tells where the files to look for are
            // located.
            // In order to avoid potential security threats, `match_assets` will never return any
            // file outside of this directory even if the URL is for example `/../../foo.txt`.

            // println!("{}", &request.url());
            // let camera_instance = camera_mutex_clone.lock().unwrap();

            // println!("req: {:?}", camera_instance.is_connected());

            let mut response = rouille::match_assets(&request, "./public");

            if &request.url() == "/" {
                let file = File::open("./public/index.html").unwrap();
                response = Response::from_file("text/html", file);
            }

            // If a file is found, the `match_assets` function will return a response with a 200
            // status code and the content of the file. If no file is found, it will instead return
            // an empty 404 response.
            // Here we check whether if a file is found, and if so we return the response.
            if response.is_success() {
                return response;
            }
        }

        // This point of the code is reached only if no static file matched the request URL.

        // In a real website you probably want to serve non-static files here (with the `router!`
        // macro for example), but here we just return a 404 response.
        Response::html(
            "404 error. Try <a href=\"/README.md\"`>README.md</a> or \
                        <a href=\"/src/lib.rs\">src/lib.rs</a> for example.",
        )
        .with_status_code(404)
    });

    
    println!("Program finished");
    
}


fn handle_websocket(data: websocket::WebSocketData) {
    
    match data {
        WebSocketData::JSON(data) => {
            println!("handle_websocket: {}", SOME_INT);
            println!("JSON way ! {:?}", data)
        },
        WebSocketData::Text(text) => println!("Old way ! {:?}", text)
    }

    // println!("Receive JSON data here {:?}", data["age"]);
}