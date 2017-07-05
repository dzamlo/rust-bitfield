#[macro_use]
extern crate simple_bitfield;

use std::net::Ipv4Addr;

simple_bitfield!{
    struct IpV4Header(MSB0 [u8]);
    get_version, _: 3, 0;
    get_ihl, _: 7, 4;
    get_dscp, _: 13, 8;
    get_ecn, _: 15, 14;
    get_total_length, _: 31, 16;
    get_identification, _: 47, 31;
    get_df, _: 49;
    get_mf, _: 50;
    get_fragment_offset, _: 63, 51;
    get_time_to_live, _: 71, 64;
    get_protocol, _: 79, 72;
    get_header_checksum, _: 95, 79;
    u8, get_source_address, _: 103, 96, 4;
    u8, get_destination_address, _: 135, 128, 4;
    }

impl<T: AsRef<[u8]> + AsMut<[u8]>> IpV4Header<T> {
    fn get_source_as_ip_addr(&self) -> Ipv4Addr {
        let mut src = [0; 4];
        for (i, src) in src.iter_mut().enumerate() {
            *src = self.get_source_address(i);
        }
        src.into()
    }

    fn get_destination_as_ip_addr(&self) -> Ipv4Addr {
        let mut dst = [0; 4];
        for (i, dst) in dst.iter_mut().enumerate() {
            *dst = self.get_destination_address(i);
        }
        dst.into()
    }
}

fn main() {
    let data = [
        0x45,
        0x00,
        0x00,
        0x40,
        0x69,
        0x27,
        0x40,
        0x00,
        0x40,
        0x11,
        0x4d,
        0x0d,
        0xc0,
        0xa8,
        0x01,
        0x2a,
        0xc0,
        0xa8,
        0x01,
        0xfe,
    ];

    let header = IpV4Header(data);

    assert_eq!(header.get_version(), 4);
    assert_eq!(header.get_total_length(), 64);
    assert_eq!(header.get_identification(), 0x6927);
    assert!(header.get_df());
    assert!(!header.get_mf());
    assert_eq!(header.get_fragment_offset(), 0);
    assert_eq!(header.get_protocol(), 0x11);
    //assert!(header.get_source_address() == [192, 168, 1, 42]);
    println!(
        "from {} to {}",
        header.get_source_as_ip_addr(),
        header.get_destination_as_ip_addr()
    );
}
