mod frame;

pub use self::frame::*;

#[cfg(test)]
mod tests {

  const TCP_ACK_PACKET: [u8; 74] = [
    0x00, 0x00, 0xc0, 0x9f, 0xa0, 0x97, 0x00, 0xa0, 0xcc, 0x3b, 0xbf, 0xfa, 0x08, 0x00, 0x45, 0x10,
    0x00, 0x3c, 0x46, 0x3c, 0x40, 0x00, 0x40, 0x06, 0x73, 0x1c, 0xc0, 0xa8, 0x00, 0x02, 0xc0, 0xa8,
    0x00, 0x01, 0x06, 0x0e, 0x00, 0x17, 0x99, 0xc5, 0xa0, 0xec, 0x00, 0x00, 0x00, 0x00, 0xa0, 0x02,
    0x7d, 0x78, 0xe0, 0xa3, 0x00, 0x00, 0x02, 0x04, 0x05, 0xb4, 0x04, 0x02, 0x08, 0x0a, 0x00, 0x9c,
    0x27, 0x24, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03, 0x03, 0x00,
  ];

  #[test]
  fn test_tcp_ack() {
    use super::ethernet::*;
    let ethernet_frame = EthernetFrame::from(&TCP_ACK_PACKET[..]);

    println!(
      "{} -> {}",
      ethernet_frame.source(),
      ethernet_frame.destination()
    );
    println!("{:#?}", ethernet_frame.type_());

    match ethernet_frame.next_layer() {
      EthernetLayer::IPv4(_ipv4_frame) => {}
      EthernetLayer::IPv6(ipv6_frame) => {
        println!("ipv6! {:?}", ipv6_frame.version());
      }
    }
  }
}
