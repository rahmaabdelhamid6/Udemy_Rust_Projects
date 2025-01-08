
//what if we want to pass green and blue only ? -> use vector slice (&[1..3])  lets us point to portion or all of the data owned by somthing else inside of our program 
fn print_elements(elements: &[String]) // this can work with either the full vector or just a portion 

// fn print_elements(elements: &Vec<String>)
{
    //we usually don't call next on iterator manually 
    //1- use a for loop automatically creates an iterator and calls next on it  
    // for element in elements // automatically create an iterator to the vector , call next on the iterator and unwrap the option that comes back , break once next retirn none
    // {
    //     println!("{}", element);
    // }
    //2-use iterator adaptors and consumers like for_each, collect, map, etc 
    //add a closure inside the for_each (iterator concumser) func -> closure is a like a function that doesn't have a name assigned to it -> (|argument| func body)
   elements.iter().for_each(|el|println!("{}", el)); // so closure func going to invoked for each element inside our vector 
    //iter are lazy nothing happens untill 1- you call next manually , 2- use a function that calls next automatically and ther are reffered to as iterator consumers becuase they consume values out of iterartor like for each... 

    //if i want to print each color twice in a line 
    elements.iter().for_each(|el|println!("{} {}", el, el));
    //or use iterator adapter -> add an extra step in a processing pipline and it doeasn't actually call next to be called 
    elements
    .iter()
    .map(|el| format!("{} {}", el, el))// to create string that contain that element twice 
    .for_each(|el|println!("{}", el));

}
// fn shorten_string(elements :&mut Vec<String>) // we can use it or the below because the below give or bit of flexibility 
fn shorten_string(elements :&mut [String]) // use vec slice (type annotation) to get the benefit of calling the function in the main with a full mutable ref to complete vec or portion of it 
{
    elements.iter_mut().for_each(|el| el.truncate(1)); // use iter_mut() instead of iter() because iter will give you a read_only ref to each element 
    //iter_mut() give you a mutable ref to each element 
    //ther is also into_iter (into word before any function means that you will take the ownership of element) -> the iterator will give you ownership of each element unless called on a mutable ref to a vector 
}
fn to_uppercase(elements :&[String]) -> Vec<String>
{
    elements.iter()
    .map(|el| el.to_uppercase())
    .collect::<Vec<String>>() // after processing each step we need to collect them all together into a single vector and it dicide it will return the data in verctor due to return typr of the function  
    // or based on the type of the variable if we assign it in a variable  or we can do it through .collect::<Vec<String>>() or .collect::<Vec<_>>() -> _ means based on the type of the data it recieved or the previous step before it
    //collect return -> vector or hashmap , double linkedlist
}
//using into_iter -> not always going to give you the ownership of each element it's going behave differently depending upon how you call this func
// always don't call it on reference or mutable ref  
fn move_elements(elements_a :Vec<String> , elements_b: &mut Vec<String> ) 
{
    elements_a.into_iter().for_each(|el| elements_b.push(el));
}
//for creating vec containing vec of strings 
fn explode (elements :&[String]) -> Vec<Vec<String>>
{
    elements
    .iter()
    .map(
        |el| el.chars().map(|c| c.to_string()).collect()
    )
    .collect()
}
fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String{
    elements
    .iter()
    .find(|el| el.contains(search)) // find is iterator consumer  calls next automatically and take a reference to some value inside of ypur vector and repeat this process untill the closure func return a truthy value then find going tp stop calling next and immediatly return an option enum some if found and ignore .map_or and none if didn't find ant map_ot will return the first argument if it have a some it will take the value inside the some is going to passed into the closure 
    .map_or(
        String::from(fallback),
        |el| el.to_string())

}
fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];
    //iter is sperate from the data structure there are two different thing -> and we make it to iterate or walk through all the different elements inside of a collection 
    let mut colors_iter = colors.iter(); // we use mut any time we expect to reassign a value in the left or we expect to modify the value on the right over and over 
    //and here we change the pointer to curent position in .iter struct every single time we call next method 
    println!("{:#?}",colors_iter.next()); //some "red"
    println!("{:#?}",colors_iter.next()); //some "green"
    println!("{:#?}",colors_iter.next()); //some "blue"
    println!("{:#?}",colors_iter.next()); //none

    print_elements(&colors);
    //what if we want to pass green and blue only ? -> use vector slice (&[1..3])  lets us point to portion or all of the data owned by somthing else inside of our program 
    print_elements(&colors[1..3]);
    // shorten_string(&mut colors);
    // shorten_string(&mut colors[1..3]);
    // println!("{:#?}", colors);

    let color_uppercase =  to_uppercase(&colors);
    println!("{:#?}", color_uppercase);

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("Destination: {:#?}", destination);

    let exploded = explode(&colors);
    println!(" exploded vec of string:{:#?}", exploded);
    
    let found_color = find_color_or(&colors,"re", "orange");
    println!("{:#?}", found_color);
    
}
