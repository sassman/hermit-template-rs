#[cfg(target_os = "hermit")]
use hermit_sys;

{% if network_enabled %}
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    loop {
        let (conn, addr) = listener.accept().unwrap();
        println!("Incoming Connection from {}", addr);
        std::io::copy(&mut &conn, &mut &conn).unwrap();
    }
}
{% else %}
fn main() {
    println!("Hello Rusty Hermit 🦀");
}
{% endif %}