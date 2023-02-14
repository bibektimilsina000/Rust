enum IpAddrKind {
    V4,
    V6,
    
}

struct IpAddr{
    kind:IpAddrKind,
    adderss:String
}

enum IpAddrKind2 {
    
    V4(String),
    V6(String),
}

enum IpAddrKid3 {
    V4(u8,u8,u8,u8),
    V6(String)
    
}


enum Message {
    Quite,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)

}




fn main(){

    let home3=IpAddrKid3::V4(223, 225, 226,  34);

    let home2=IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback2=IpAddrKind2::V6(String::from("::1"));


    let home=IpAddr{
        kind:IpAddrKind::V4,
        adderss:String::from("127.0.0.1"),

    };

 
    let loopback=IpAddr{
        kind:IpAddrKind::V6,
        adderss:String::from("::1")
    };
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;



     
}