use std::{
    io,
    net::{Shutdown, SocketAddr},
    time::Duration,
};

use hyper::Client;
use tokio::{net::TcpStream, time};

use crate::error;

pub struct HttpAddr {
    pub host: String,
    pub port: String,
    pub path: String,
}

impl HttpAddr {
    pub fn format(&self) -> String {
        format!("http://{}:{}{}", self.host, self.port, self.path)
    }

    pub async fn ping(&self) -> Result<(), ()> {
        let client = Client::new();
        let url = self.format();
        let uri = url.parse().expect(&format!("Failed to parse url: {}", url));
        match client.get(uri).await {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        }
    }
}

pub struct TcpAddr {
    pub host: String,
    pub port: String,
}

impl TcpAddr {
    fn format(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub async fn wait(&self) -> io::Result<()> {
        let addr = self.format();
        match addr.as_str().parse::<SocketAddr>() {
            Ok(addr) => {
                loop {
                    match TcpStream::connect(addr).await {
                        Ok(stream) => {
                            if let Err(error) = stream.shutdown(Shutdown::Both) {
                                eprintln!("Failed to close socket: {}", error);
                            };
                            break;
                        }
                        Err(_) => time::sleep(Duration::from_millis(250)).await,
                    }
                }
                time::sleep(Duration::from_millis(1500)).await; // giving some time to warm up
                Ok(())
            }
            Err(error) => Err(error::other(format!(
                "Failed to parse addr \"{}\": {}",
                addr, error
            ))),
        }
    }
}
