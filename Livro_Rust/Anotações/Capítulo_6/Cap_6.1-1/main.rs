struct IpV4Addr {
    // -- snip --
}

struct IpV6Addr {
    // -- snip --
}

enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

struct QuitMessage;

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);

struct ChangeColorMessage(i32, i32, i32);

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(127, 0, 0, 1);

    let loop_back = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from4("hello"));

    m.call();
}

fn route(ip_kind: IpAddrKind) {}