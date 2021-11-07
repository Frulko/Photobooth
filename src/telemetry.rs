

pub struct Telemetry {
  is_connected: bool,
}

impl Telemetry{

  pub fn new() -> Telemetry {
    let mut tl = Telemetry {
      is_connected: false,
    };

    tl.connect();

    return tl;
  }

  fn log() {
    // TODO: Can be seperate by log identifier
  }

  fn send() {

  }

  fn get_latest() {
    // TODO: retrive and send last 10 log records
  }
}