fn main() {
    // Scalar represents 4 data types in Rust
    // like Integer,Floating point, Boolean and Character

    //example of usigned Integer


    //error[E0600]: cannot apply unary operator `-` to type `u32`

    
   // This line will prompt above error -> let x:u32 = -12;

    let x:u32 = 12;

    println!("Unsigned number {}",x);

    let y:i32 = -12;

    println!("Signed number {}", y);


    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    //error[E0277]: `(i32, f64, u8)` doesn't implement `std::fmt::Display`

//    let (x, y, z) = tuple;
//   println!("Tuple example {}", y);

//Destructuring example in Rust tuple example
//You can access to the initialized value of tuple by "."
//
     let five_hundred = tuple.0;
     let six = tuple.1;
     let one = tuple.2;

     println!("Tuple accessed variables by destructuring {},{},{}",five_hundred,six,one);


// Array is single chunk of memory allocated on stack
// which is preferable to use during initialisation of 
// months number where we are sure that it will end with
// number 12.
//
// To initialise array you have to declare 
//
   let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
  let first_month = months[0]; 
  let zero:i32 = 0;
   format!("Array of months example with {}",first_month);
   format!("with access to first position which is in this example {}",zero);

   //let out_of_bounds = months[12];
   //|                        ^^^^^^^^^^ index out of bound//    s: the length is 12 but the index is 12
//    println!("Array out of bounds {}", out_of_bounds);

}
