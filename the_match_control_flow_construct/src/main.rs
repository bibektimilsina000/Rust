
enum Coin{
    Penny,
    Nickle,
    Dime,
    Quater(UsState),
}
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}




fn main() {
     
}
 
fn value_in_cents(coin:Coin) -> u8{

    match coin {
        
        Coin::Penny=>{
            println!("Lucky_Penny");
            1
        }

        Coin::Nickle=>5,
        Coin::Dime=>10,
        Coin::Quater(state)=>{

            println!("state qatar from {:?}",state);
            25
        }
    }


}