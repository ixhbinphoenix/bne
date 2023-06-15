use std::{net::{IpAddr, SocketAddr}, str::FromStr, fmt::Display};
use actix_governor::{KeyExtractor, SimpleKeyExtractionError};
use actix_web::web;
use log::{debug, error, info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct NginxIpKeyExctrator;

macro_rules! couldntExtract {
    () => {
        actix_governor::SimpleKeyExtractionError::new("Could not extract real IP Address from request")
    };
}

impl KeyExtractor for NginxIpKeyExctrator {
    type Key = IpAddr;

    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn name(&self) -> &'static str {
        "Proxy IP"
    }

    fn extract(&self, req: &actix_web::dev::ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        let proxy_ip = req
            .app_data::<web::Data<IpAddr>>()
            .map(|ip| ip.get_ref().to_owned())
            .unwrap_or_else(|| IpAddr::from_str("0.0.0.0").unwrap());

        let peer_ip = req.peer_addr().map(|socket| socket.ip());
        let connection_info = req.connection_info();

        info!("Proxy IP: {}", proxy_ip);

        match peer_ip {
            Some(a) => {
                info!("Peer IP: {}", a);
            }
            _ => {
                info!("Peer IP: None");
            }
        }

        match peer_ip {
            // request is from reverse proxy, so use 'Forwarded' or 'X-Forwarded-For' header
            Some(peer) if peer == proxy_ip => connection_info
                .realip_remote_addr()
                .ok_or_else(|| { couldntExtract!() })
                .and_then(|str| {
                    SocketAddr::from_str(str)
                        .map(|socket| socket.ip())
                        .or_else(|_| IpAddr::from_str(str))
                        .map_err(|_| { couldntExtract!() })
                }),
            Some(peer) => {
                if cfg!(not(debug_assertions)) {
                    if peer.to_string() != "127.0.0.1" {
                        error!("!!!FATAL!!! SERVER MISCONFIGURED, GOT REQUEST FROM REVERSE PROXY DIRECTLY");
                        panic!();
                    }
                }
                connection_info
                    .peer_addr()
                    .ok_or_else(|| { couldntExtract!() })
                    .and_then(|str| {
                        SocketAddr::from_str(str).map_err(|_| {couldntExtract!()})
                    })
                    .map(|socket| socket.ip())
            }
            _ => {
                if cfg!(not(debug_assertions)) {
                    error!("!!!FATAL!!! SERVER MISCONFIGURED, GOT OUTSIDE REQUEST NOT THROUGH PROXY");
                    panic!();
                }
                connection_info
                    .peer_addr()
                    .ok_or_else(|| { couldntExtract!() })
                    .and_then(|str| {
                        SocketAddr::from_str(str).map_err(|_| {couldntExtract!()})
                    })
                    .map(|socket| socket.ip())
            }
        }
    }

    fn key_name(&self, key: &Self::Key) -> Option<String> {
        Some(key.to_string())
    }
}
