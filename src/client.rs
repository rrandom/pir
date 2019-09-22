

pub struct Client {
  serverAddr: String,
  tcpNum: isize,
}

impl Client {
  fn new(serverAddr: String, tcpNum: isize) -> Self {
    Client {
      serverAddr,
      tcpNum,
    }
  }
}

#[test]
fn test_create() {
    let c = Client::new("localhost".to_string(), 1);
    assert_eq!(c.serverAddr, "localhost".to_string());
    assert_eq!(c.tcpNum, 1);
}
