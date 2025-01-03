use std::{ fs, result};
use std::io::Error; //used to represent different kind of errors that occur inside of our program
//enum are kind of like a shortcut for defining multiple structs all in one go , creating some of data sytucture in memory
fn extract_error(text: &str) -> Vec<&str> //using string slice because we don't want to take the ownership we only will do some processing
{
    let split_text = text.split("\n"); //take the text and seperate it in individual lines 
    let mut results= vec![];
    for line in split_text {
        if line.starts_with("ERROR"){
            results.push(line);
        }
    }
    results
}
// fn extract_error(text: &str) -> Vec<String> //using string slice because we don't want to take the ownership we only will do some processing
// {
//     let split_text = text.split("\n"); //take the text and seperate it in individual lines 
//     let mut results= vec![];
//     for line in split_text {
//         if line.starts_with("ERROR"){
//             results.push(line.to_string());
//         }
//     }
//     results
// }

fn main() -> Result<(), Error>{
// fn main() {
    // let text = fs::read_to_string("logs.txt");
    // //OK thing is another thing very similar enum that is being used to do a little bit of error handling
    // println!("{:#?}", text);
//    let mut error_logs = vec![];
    //1- first way to handle the esult eenum
    // match fs::read_to_string("logs.txt"){
    //     Ok(text_that_was_read) =>{
    //         // println!("{}", text_that_was_read.len()) //print out the number of charachters
    //         let error_logs = extract_error(text_that_was_read.as_str()); // now the ead_to_string func return an string but in the func we made it take string slice &str 
    //         // error_logs = extract_error(text_that_was_read.as_str()); 
    //         // we have two ways to take the string and convert it into string slice 
    //         //1- put & before it (&text_that_was_read) 
    //         //2- call the .as_str() method text_that_was_read.as_str()
    //         println!("{:#?}", error_logs);
    //         match fs::write("error.txt", error_logs.join("\n"))// this is an operationtion that might succeed or fail 
    //         {
    //             Ok(..) => println!("wrote error.txt"),
    //             Err(reason_write_failed) => {
    //                 println!("writing of errors.txt failed: {}", reason_write_failed);
    //             }
    //         }
    //         //when text_that_was_read goes out of scope , its value going to be dropped  -> so after the println!() it dropped not accessible anymore
    //         // so to if i declare the error_logs first as empty vec , then take the println!() outof the match  we will get that error ( does not live long enough borrowed value does not live long enough)
    //         // it's because of the split func in extract_error func() it returns Vec<&str> and this have an affect on the code so we will fix this func
    //     }
    //     Err(why_this_failed) => {
    //         println!("failed to read file: {}", why_this_failed)
    //     }
    // }
    // println!("{:#?}", error_logs);

    //2-result enum also has a set of method like option enum (unwrap , except, unwrap_or) allow you to get access to values inside the enum 
    //the difference between this code and the above of the match statement is if any of these operations fail, we are now going to panic and is going to essentially crash our entire program 
    // let text = fs::read_to_string("logs.txt").expect(
    //         "failed to read logs.txt"); // if we read the file successfully and we get back the okay variant -> the text inside of the ok variant will be automatically assigned to this text variable right 
    //     // text now is -> the actual raw text , but if we failed to read the file and we get back the error variant then expect is going to cause a panic (include the error msg inside of it )
    //     // println!("{:#?}", text);
    //     let error_logs = extract_error(text.as_str());
    //     fs::write("error.txt", error_logs.join("\n"))
    //     .expect("failed to write error.txt"); 

    //3- merge 1-return reult enum from the main & 2- add new operator (?) to the very end of a func that return a result 
    // this operator ? do something depend on whether or not this function gives us back the okay variant or the error variant 
    //if the return from the func is ok it will assign the result to whatever variable 
    //if error it will do somthing different , it is going to automatically unwrap the value inside of there , then automatically return that value early so no assign occur nothing else inside the main func will be executed 
    // Ok(());
    // Err(Error::other("something went wrong.."));
    let text = fs::read_to_string("logs.txt")?;
    // println!("{:#?}", text.len()); //to verify that we get all the text successfully by printing the number of charachters in that text
    let error_logs = extract_error(text.as_str());
    fs::write("error.txt", error_logs.join("\n"))?;
    Ok(()) // to check the fail change the file name run the code it will print error msg and won't execute the println or ok(())
    // match divide(5.0, 0.0){
    //     Ok( result_of_division) =>{
    //         println!("{}", result_of_division);
    // }
    //     Err(what_went_wrong) => {
    //         println!("{}", what_went_wrong);
    //     }
    // }
    // match validate_email(String::from("rahamaabdelhamid@gmail.com")) {
    //     Ok(..) => println!("email is valid"), //.. maeans that we aclnowledge that there is a value inside of tha okay variant 
    //     Err(reason_this_failed_validation )=>{
    //         println!("{}", reason_this_failed_validation )
    //     }
        
    // }
}
// fn validate_email(email: String) ->Result<(), Error>
// {
//     if email.contains("@")
//     {
//         Ok(()) // empty parentheses and what exactly is an empty tuple when we put () we are creating an instance of a tuple and it  -> if we don't have any meaningful data to return inside the okay variant  
//         // the difference between tuples and struct is that tuple don't have explicity label just a type  
//     }else {
//         Err(Error::other("email must have @"))
//     }
// }

// fn divide (a:f64 , b:f64) -> Result<f64, Error> //<like func arg but for types instead> means its generic enum that we can kind of use different kinds of types in this enum
// {
//     if b == 0.0{
//         Err(Error::other ("can't divide by 0"))// Err -> is a variant E in Result <T, E> ,//Error::other == Bank::new()-> create an instance of of the error struct to use to kind of communication or indicate that somthing wrong has occurred inside of our program 
//     }
//     else {
//         Ok(a/b) //creating new instance of the variant
//         //what if we are doing some other kind of thing that doesn't really have any kind of a natural data output tied to it like if try to write to a file , removing a directory , permission check , validating a string 
//     }
// }