 

fn main() {
    //intiger signed and unsiged

    let num:u32=334; //32 bit unsigned intiger
    let snum:i32=-323; //32 bit signed intiget

    //floating point floating point are of signed only

    let x=2.0; // f64 bit by deafult
    let y:f32 =3.0; // f32 bit

    //Numeric oprators in rust

    let sum = 5 + 10;


    let difference=30.3-4.6;

    let product = 4*54;

    //division
    let quetient=56.7/32.2;
    let truncate=-5/3; //result -1

    //reminder

    let reminder=43%4;



    //The Boolen type 


    let t=true;

    let f: bool=false;

    //char

    let z='z';
    let c:char='z';


    //Cmpound types

        //tuppule : length of tuple cannot be increased 
        

    let tup:(i32,f64,u8)=(500,6.4,3);


    let tup2 =(500,6.4,2);
    let (x,y,z)=tup2;

    println!("the vaule of y is : {y}");

    // we can also acess tuple elemnt by using . index;

    let file_hundred=tup2.0;
    let six_point_four=tup2.1;
    let two = tup2.2;


    // The Array Type

    let a = [1,2,3,4];

let month = ["Jan","Feb","March","April","May","June"];

    //Acessing array element

    let first = month[0];
    let second=month[1];

    

















    



}
