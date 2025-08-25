pub fn main() {
    println!();
    println!("==================================================");
    println!("Enums ");
    println!("==================================================");

    enum IpAddrKind {
        V4,
        V6,
    }
    let ip_kind = IpAddrKind::V4;

    fn route(ip_kind: IpAddrKind) {
        if let IpAddrKind::V4 = ip_kind {
            println!("route to V4");
        } else if let IpAddrKind::V6 = ip_kind {
            println!("route to V6");
        }
    }
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // using struct
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: "127.0.0.1".to_string(),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: "::1".to_string(),
    };

    // using enums
    enum IpAddrEnum {
        V4(String),
        V6(String),
    }
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));

    enum IpAddrEnum2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home2 = IpAddrEnum2::V4(127, 0, 0, 1);
}
