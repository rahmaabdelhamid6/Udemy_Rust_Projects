// 1- ru n this command cargo add num-traits
// it's not allowed to do any kind of arithmetic between different types of numbers , even if they're both float but one 32 and another 64 same for int and unsigned int as well 
// geneic types <T: Float> (T is just a name and could be replaced with anything like U, K, etc ) -> allows us to provide some customization or really flexibility arounf not only func but methods , enums, vectors, structs. to work with different types 

use num_traits::{Float, ToPrimitive}; // it add some additional methods to all different types of numbers in rust

//1- first version: we can pass in both f32 or f64 by using Generic Types <T: Float> -> fn solve<T: Float>(a: T, b:T) ->f64 and add float in use num_traits
// float i <T: Float> is a trait bound , trait is a set of methods it contains a abstract method which don't have an implementation , and default methods which have an implementation 
// a struct, enum , primitive can implement a trait and the implemntor has to provide an implementation for all the abstract methods and can optionally override the default method
// when a struct or whatever imoplement the trait it get two benefits 1- the struct for ex car considerd to be of type of the trait vecichle for ex , 2- it gets access to all different ,methods wthat where defined inside the trait 
// what if i wnat to be able to pass f32 and f64 ? fn solve<T: Float, U:Float>(a: T, b:U) ->f64 
// fn solve<T: Float, U:Float>(a: T, b:U) ->f64 // when you call solve ther's going to be two different generic types -.fn solve<f32: Float, f64:Float>(a: f32, b:f64) ->f64
// fn solve<T: Float>(a: T, b:T) ->f64 // f64 and f32 implement float trait 
// {
//     let a_f64 = a.to_f64().unwrap();
//     let b_f64 = b.to_f64().unwrap();
    
//     (a_f64.powi(2) + b_f64.powi(2)).sqrt()
// }
// 2- second version:  we can pass in any type of numbers by change Float to ToPrimitive
fn solve<T: ToPrimitive, U:ToPrimitive>(a: T, b:U) ->f64 // any type implementation , ToPrimitive is a trait and it adds some additiona;l functionality to all the different number types
{
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

//default
// fn solve(a: f64, b:f64) ->f64
// {
//     (a.powi(2) + b.powi(2)).sqrt()
// }
fn main() {
    //default
    // let a: f32= 3.0;// by default rust intiate float as f64 if i changed it to f32 i will se an error and to fix this we need to convert it to f64 and wew can do this with many ways and we will discuss two of them
    //1- let a_f64 = a as f64; // and pass a_f64 to the solve func 
    //2- using num crates  -> 1- use num_traits::ToPrimitive; 2- let a_f64 = a.to_f64().unwrap();
    //3- we can add the line in number 2 inside the solve func to avoid repeating it ahead of time every single time we call solve 
    // let a_f64 = a.to_f64().unwrap(); // when we call to_f64 it return an Option enum to handle overflows so we need to call .unwrap()
    // let b:f32 = 4.0;
    // println!("{}",solve(a, b));


    //1- solve<T: Float>(a: T, b:T) ->f64 , solve<T: Float, U:Float>(a: T, b:U) ->f64 
    // let a:f32= 3.0;
    // let b:f32 = 4.0;
    // println!("{}",solve::<f32>(a, b));
    // println!("{}",solve(a, b)); // both works it's not actually strictly required
    // //2- solve<T: Float>(a: T, b:T) ->f64
    // let a:f64= 3.0;
    // let b:f64 = 4.0;
    // println!("{}",solve::<f64>(a, b));
    // println!("{}",solve(a, b));

    //2-
    let a:i32= 3;
    let b:f32 = 4.0;
    println!("{}",solve::<i32,f32>(a, b));
    println!("{}",solve(a, b)); // both works it's not actually strictly required
    //2- solve<T: Float>(a: T, b:T) ->f64
    let a:i64= 3;
    let b:f64 = 4.0;
    println!("{}",solve::<i64,f64>(a, b));
    println!("{}",solve(a, b)); 




   
}
