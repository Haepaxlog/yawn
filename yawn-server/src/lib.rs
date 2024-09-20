use pnet::{
    datalink::{self, NetworkInterface},
    ipnetwork::IpNetwork,
};
use std::time::Duration;

struct Server {
    poll_interval: Duration,
}

impl Server {
    pub fn setup(poll_interval: Duration) -> Self {
        // TODO: Device Network Interfaces ips should be checked asynchronously every poll_interval in case network ifaces
        // are activated or deactivated while the server is running
        // TODO: The server should also whenever different ifaces are added or dropped in the new scan
        // subscribe or unsubsribe to nats subject "yawn.outposts.<ip>/<mask>" of their respective ips
        Server { poll_interval }
    }

    fn device_network_ifaces(&self) -> Vec<NetworkInterface> {
        datalink::interfaces()
    }

    fn ip_addresses<F>(&self, filter: F) -> Vec<IpNetwork>
    where
        F: Fn(&IpNetwork) -> bool,
    {
        self.device_network_ifaces()
            .clone()
            .into_iter()
            .flat_map(|x| {
                x.ips
                    .into_iter()
                    .filter(|ip| !ip.ip().is_loopback() && filter(ip))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn ipv4_addresses(&self) -> Vec<IpNetwork> {
        self.ip_addresses(|ip| matches!(ip, IpNetwork::V4(_)))
    }

    fn ipv6_addresses(&self) -> Vec<IpNetwork> {
        self.ip_addresses(|ip| matches!(ip, IpNetwork::V6(_)))
    }
}
