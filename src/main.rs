use std::env;
use std::net::ToSocketAddrs;
use url::Url;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ipu <url>");
        std::process::exit(1);
    }

    let input = &args[1];
    
    match url_to_ip(input) {
        Ok(ip) => println!("Ip: {}", ip),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn url_to_ip(input: &str) -> Result<String, String> {
    // Parse the URL
    let url = Url::parse(input)
        .map_err(|_| "Invalid URL format")?;
    
    // Extract the host
    let host = url.host_str()
        .ok_or("No host found in URL")?;
    
    // Get the port (default to 80 for http, 443 for https)
    let port = url.port().unwrap_or_else(|| {
        match url.scheme() {
            "https" => 443,
            _ => 80,
        }
    });
    
    // Resolve hostname to IP address
    let socket_addr = format!("{}:{}", host, port);
    let mut addrs = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve host: {}", e))?;
    
    // Get the first IP address
    let addr = addrs.next()
        .ok_or("No IP address found")?;
    
    Ok(addr.ip().to_string())
}
