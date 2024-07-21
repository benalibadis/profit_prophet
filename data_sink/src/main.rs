use std::net::SocketAddr;
use connector::influxdb::InfluxDbClient;
use data_sink::influxdb::TcpInfluxDbServer;


#[tokio::main]
async fn main() {
    // Create an InfluxDbClient instance
    let influxdb_client = InfluxDbClient::new("http://localhost:8086", "KAtopm-k4IVJUagoQm_obwIZF1WyRfSTzlHIGVkBc91otqztpOjkLnbAu8WJ9H7hVZjiOBSDvlNztkWKpY1OVQ==");

    // Define the address to bind the TCP server to
    let address: SocketAddr = "127.0.0.1:12345".parse().expect("Invalid address");

    // Create the TCP server instance
    let server = TcpInfluxDbServer::new(influxdb_client, address);

    // Run the server
    if let Err(e) = server.run().await {
        eprintln!("Server error: {:?}", e);
    }
}
