#[allow(unused)]
#[derive(Debug)]
enum IpAddr {
    IpV4(IpV4),
    IpV6(IpV6),
}

#[allow(unused)]
#[derive(Debug)]
enum IpV4 {
    Some(u8, u8, u8, u8),
    None,
}

#[allow(unused)]
#[derive(Debug)]
enum IpV6 {
    Some(String),
    None,
}

fn main() {
    let ipv4: IpAddr = IpAddr::IpV4(IpV4::Some(127, 0, 0, 1));
    let ipv6: IpAddr = IpAddr::IpV6(IpV6::Some(String::from("::1")));

    println!("{ipv4:?}");
    println!("{ipv6:?}");

    // For a specific one check
    {
        if let IpAddr::IpV4(v4) = &ipv4 {
            match v4 {
                IpV4::Some(a, b, c, d) => {
                    println!("This variable contains an IpV4 address: \"{a}.{b}.{c}.{d}\"")
                }
                _ => println!("Missing value"),
            }
        }
    }

    // For overall check
    {
        match ipv6 {
            IpAddr::IpV4(IpV4::Some(a, b, c, d)) => {
                println!("This variable contains an IpV4 address: \"{a}.{b}.{c}.{d}\"")
            }
            IpAddr::IpV4(IpV4::None) => println!("Missing IpV4 value"),

            IpAddr::IpV6(IpV6::Some(content)) => {
                println!("This variable contains an IpV6 address: \"{content}\"")
            }
            _ => println!("Missing IpV6 value"),
        }
    }
}
