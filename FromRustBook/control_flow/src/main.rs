

fn main() {
    was_true();
    is_number_three();
    is_not_zero();
    six_eater();
    if_in_let();
}

fn was_true() {
    let num = 3;

    if num < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }
}


fn is_number_three(){
    let number=3;
    if number==3{
        println!("number was three")
    }
}

fn is_not_zero(){
    let number=4;

    if number!=0{
        println!("number was something other then zero");
    }
}

fn  six_eater() {

    let number=6;

    if number%4==0{
        println!("number is divisible by 4");
    }else if number%3==0 {
        println!("number is divisible by 3");
    }else if number%2==0 {
        println!("number is divisible by 2");
    }else {
        println!("number is not divisible by 4,3 or 2");
    }
    
}

fn if_in_let(){
    let condition=false;
    let number = if condition {5 }else{6};
    println!("The value of number is : {number}")
}
