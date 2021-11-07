use std::{
  net::{TcpListener, TcpStream},
  thread::spawn,
};
use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message, Result};
use serde_json;
use serde_json::json;


pub enum WebSocketData {
  JSON(serde_json::Value),
  Text(String)
}

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
  match err {
      HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
      HandshakeError::Failure(f) => f,
  }
}




fn handle_client(stream: TcpStream, callback: fn(obj: WebSocketData)) -> Result<()> {
  let john = json!({
    "name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
  });

  let mut socket = accept(stream).map_err(must_not_block)?;

  loop {
      match socket.read_message()? {
          msg @ Message::Text(_) | msg @ Message::Binary(_) => {
            let msg_data = msg.into_text().unwrap();

            match serde_json::from_str(msg_data.as_str()) {
              Ok(t) => {
                callback(WebSocketData::JSON(t));
                socket.write_message(Message::Text(john.to_string().into())).unwrap();
              },
              Err(_) => { // fallback behavior classic Text
                // socket.write_message(Message::Text("oldway".into())).unwrap();
                callback(WebSocketData::Text(msg_data));
                socket.write_message(Message::Text("Hello WebSocket".into())).unwrap();
              }
            }
          }
          Message::Ping(_) | Message::Pong(_) | Message::Close(_) => {}
      }
  }
}

pub fn start(host: &str, port: i32, callback: fn(obj: WebSocketData)) {
  // let host = "127.0.0.1";
  let address = format!("{}:{}", host, port);

  println!(">> websocket start at: {}", address);
  
  let server = TcpListener::bind(address.as_str()).unwrap();
  for stream in server.incoming() {
    spawn(move || match stream {
      Ok(stream) => {

        if let Err(err) = handle_client(stream, callback) {
          match err {
            Error::Protocol(_) | Error::Utf8 => (),
            Error::ConnectionClosed => {
              println!("ConnectionClosed:");
            },
            e => println!("test: {}", e),
          }
        }
      }
      Err(e) => println!("Error accepting stream: {}", e),
    });
  }
    
}