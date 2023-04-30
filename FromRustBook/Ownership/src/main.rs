fn main() {
    {
        let s: String = String::from("Hello");
        let num = 5;

        let x = 5;
        let y = x;
        println!("x={},y={}", x, y);

        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1={}, s2={}", s1, s2);

        fn take_ownership(some_string: String) {
            println!("{}", some_string);
        }

        take_ownership(s);
        //using s variable below this throw error becouse ownership of s is passed to argument of function take_ownership
        // println!("{}",s);
        // Here, some_string goes out of scope and `drop` is called. The backing
        // memory is freed.

        fn make_copy(some_intiger: i32) {
            println!("{}", some_intiger);
        }

        make_copy(num);

        // we can still use num below this function call

        // All the integer types, such as u32.
        // The Boolean type, bool, with values true and false.
        // All the floating-point types, such as f64.
        // The character type, char.
        // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    }
    {
        // ownership  using function

        let s1 = give_ownership();
        println!("after give_ownership s1= '{}'", s1);

        let s2 = String::from("hello");
        let s3: String = take_and_give_back(s2);
        println!("after giving and taking back ownership of '{}' ", s3);

        let (s4, len) = calcualte_len(s3);

        println!("string is '{}' length of str is '{}'", s4, len);
    }

    {
        //Refrences and borrowing

        let s1 = String::from("hello");
        let len = calc_len_ref(&s1);
        println!(
            "s1 can be still used here becuse we pass the refrence of the variable ' s1={}'",
            s1
        );
        println!(
            "after calling the functin calc_len_ref() we can find length for s1 lenofs1={}",
            len
        );
    }

    //what happen if we want to change the variable that we borrow

    {
        let mut s1 = String::from("Some String");
        change(&mut s1);

        // let   s1=String::from("Some String");
        // change(&  s1);
        //it show error in below function becouse varaible are immutiable by defult so are refrence  ,
        // we can solve the issue by making s1 mutable first and pass mutable refrence to change and
        // accept mutable refrence in  parameter of change function

        // mutable refrence have one big restriction i.e. they can be refrence only once

        // below example throw errors

        let mut s2 = String::from("hello");

        {
            let r1 = &mut s2;
            println!("{}", r1);
        } //r1 goes out of scope here

        // let r2 = &mut s2;

        //above line give error

        //we can always use scope and create multiple mutable refrences but not simultisonly

        let r2 = &mut s2;

        println!("{}", r2);
    }

    {
        //we cannot have mutable refrence while we have immutable refrence in same scope

        let mut str = String::from("hello");

        let r1: &String = &str;
        let r2: &String = &str;
        // let r3= &mut str;
        //above line throw error
        println!("{},{}", r1, r2);

        //but we can create mutable refrence below here becouse scope of r1,and r2 ends becouse above line is the last time where
        // r1 and r2 are used

        let r3 = &mut str;
        println!("{}", r3)
    }

    //the slice type

    {
        let mut s1 = String::from("Hello World");

        let space_index = first_word(&s1);
        s1.clear();
        println!("{}", space_index);
    } //here above block contain logical error but at compile time it runs sucessfully
      // here after clearing the s1 there is still value in space_index which is but the string is totally invalid so
      // To prevent those logical error we use string slicing
      // lets see how compailer catch the error at compile time if we use slice typee in following block

    {
        let /* mut */ s1 = String::from("hello world");

        let first_word = first_word_using_string_slicing(&s1);
        //now if i try to clear the string s1 compailer will throw error
      //  this line will give error >>>   s1.clear();

        println!("first word is {}", first_word)
    }
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn take_and_give_back(a_string: String) -> String {
    println!(
        "taking ownership of '{}' and returing back to variable which calls this function",
        a_string
    );

    a_string
}

fn calcualte_len(str: String) -> (String, usize) {
    let len = str.len();
    (str, len)
}

//function for refrencing

fn calc_len_ref(s: &String) -> usize {
    println!("this statment is from inside the function and value of s1 can still be used here from refrense as s s={}",s);

    s.len()
}



//tryng to change borrow variable

fn change(s: &mut String) {
    s.push_str("added string to some string");
}

//slice with out using string slicing type

fn first_word(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (index, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return index;
        }
    }

    bytes.len()
}

// lets see how above error/bug is prevented if we use stirng slice typee

fn first_word_using_string_slicing(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (index, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return &str[..index];
        }
    }
    &str[..]
}
