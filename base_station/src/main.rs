#[macro_use]
extern crate log;
extern crate env_logger;
extern crate getopts;
extern crate smoltcp;

mod utils;

use std::str;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::os::unix::io::AsRawFd;
use smoltcp::phy::wait as phy_wait;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr};
use smoltcp::iface::{NeighborCache, EthernetInterfaceBuilder};
use smoltcp::socket::SocketSet;
use smoltcp::socket::{UdpSocket, UdpSocketBuffer, UdpPacketMetadata};
use smoltcp::socket::{TcpSocket, TcpSocketBuffer};
use smoltcp::time::{Duration, Instant};

fn main() {
    // utils::setup_logging("");

    let (mut opts, mut free) = utils::create_options();
    utils::add_tap_options(&mut opts, &mut free);
    utils::add_middleware_options(&mut opts, &mut free);

    let mut matches = utils::parse_options(&opts, free);
    let device = utils::parse_tap_options(&mut matches);
    let fd = device.as_raw_fd();
    let device = utils::parse_middleware_options(&mut matches, device, /*loopback=*/false);

    let neighbor_cache = NeighborCache::new(BTreeMap::new());

    let tcp1_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp1_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp1_socket = TcpSocket::new(tcp1_rx_buffer, tcp1_tx_buffer);

    let ethernet_addr = EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
    let ip_addrs = [
        IpCidr::new(IpAddress::v4(192, 168, 69, 100), 24)
    ];
    let mut iface = EthernetInterfaceBuilder::new(device)
            .ethernet_addr(ethernet_addr)
            .neighbor_cache(neighbor_cache)
            .ip_addrs(ip_addrs)
            .finalize();

    let mut sockets = SocketSet::new(vec![]);
    let tcp1_handle = sockets.add(tcp1_socket);

    loop {
        let timestamp = Instant::now();
        match iface.poll(&mut sockets, timestamp) {
            Ok(_) => {},
            Err(e) => {
                debug!("poll error: {}", e);
            }
        }

    

        // tcp:8000: respond "hello"
        {
            let mut socket = sockets.get::<TcpSocket>(tcp1_handle);
            if !socket.is_open() {
                socket.listen(8000).unwrap();
            }

            // if socket.can_send() {
            //     debug!("tcp:8000 send greeting");
            //     write!(socket, "{{ status: \"ok\" }}").unwrap();
            //     debug!("tcp:8000 close");
            //     socket.close();
            // }
        }

        phy_wait(fd, iface.poll_delay(&sockets, timestamp)).expect("wait error");
    }
}