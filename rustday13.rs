
enum IpAddrKind{
    V4(u8, u8, u8, u8,),
    V6(String),
}

enum Message{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)

}

impl Message{
    fn some_function(){
        println!("Titsandberr")
    }
}
struct IpAddr{

    kind:IpAddrKind,
    addres:String,
}
fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

// let localhost = IpAddrKind::V4(String::from("127.0.0.1"));
let localhost = IpAddrKind::V4(127, 0, 0, 1);

}

fn route(ip_kind:IpAddrKind){

}
