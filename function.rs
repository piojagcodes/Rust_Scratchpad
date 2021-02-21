fn main() {
    /*
     By keyword "fn" you can initialize function
     which can held either instructions or arguments
     (shorthand from concrete values)
    
     */
    
    /*
      fn name_of_function() {
      println!("Prompted out message");
     }
    */


    
    println!("Basic message");
    this_time_not_main(5,6);
    let p = five_o();

    /*
      Statements are instructions that hold value 
      which is not used during evaluation 
      process.
    */


    let x:u32 = 256;

    
    // Initialising variable within another block
    let z = {
        let j = 23;
        j + 1
    };
    println!("Variable initialised within another block {}", z);
    println!("Variable prompted from function invoked by expression {}", p);




}

 /*
 *  Rust is an expression-based language which enables as feature to write function with return value
 *  
 *  Please notice that lack of semicolon near "50" is not accidental
 *
 *  If you place semicolon you will cause compiler to prompt error number E308
 *  which refers to using innapropriate type during matching type process. 
 *
 *  fn function_name() // <- empty tuple  -> i32 <- data type 
 *  {
 *                      FUNCTION'S BODY
 *
 *      50; <- returned value in mismatched type
 *      
 *      50 <- correct value in function's body
 *
 *  }
 *
 * 
 */
    fn five_o() -> i32 {
        50;

    }


 fn this_time_not_main(x:u32, y:u32) {

     /*
     
     Below there are examples of expression form
     in method's body.
     
     They are quite different than statements because
     of returning values abillity.
     */

     println!("Variable X holds value {}", x);
     println!("Variable Y holds value {}", y);



}
