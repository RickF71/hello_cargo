use std::{io, net::IpAddr};

use pcap::{
    PcapHandle,
    PcapFilterBuilder,
    PcapPacket,
    PcapResult,
    PcapType,
};

fn main() -> PcapResult<()> {
    use pcap::Device;

    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();

    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}", packet);
    }
}
