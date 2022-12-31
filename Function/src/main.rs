fn main() {
    another_function(56, 'h');
    scope_func();
    let five = five();
    println!("The value of five is : {five}");

    let x = plus_one(5);
    println!("The value of x is : {x}");
}

fn another_function(value: i32, label: char) {
    println!("input value {value} input label {label}");
}

fn scope_func() {
    let y = {
        let x = 3;
        x + 2
    };
    println!("the value of y is {y}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
