use library::{bytes::{Bytes, IntoBytes}, prelude::{Packet, PacketKind}};

#[test]
fn packet_from_bytes() {
    let packet_bytes = Bytes::from_arr([0, 4, 0, 0]);

    let empty_packet = Packet::new(PacketKind::Empty, Bytes::new()).into_bytes();

    assert_eq!(packet_bytes.into_vec(), empty_packet.into_vec()); 
}