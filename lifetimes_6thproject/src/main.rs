

// you really only have to think about annotation in general, anytime your function recieve and return a ref 
//adding a lifetime annotation doesn't change how your code runs at all or it doesn't prolong a ref , it doesn't make it live longer or anything like that 
// it is slowly communicating the relationship between how the returned ref right here and wether this ref is pointing at this arg or this arg 
//life time annotation used with functions, struct, enums, and more  ('a) -< describe how long some kind of value is going to live before it gets cleanup automatically as it goes out of scope 
// the goal of it ia to make the compiler make sure refs won't outlivete value they refer to , the hardest part in understanding annotation is just understanding why do we have to add them in ourselves 
// 1- any time we make a func that take two references or more and return a ref rust is going to do huge assumption -> that the return will point at data reffered to by one of the argument so we need to use a lifetime annotation 
//2- rust will not analyze the body of your func to figure out whether the return ref is pointing at the first or the sec arg -> so with the lifetime annotation we will tell rust whether this returned reference right here is pointing to the first or sec arg 
// so we will put <'a> beside the func name as (a) is identifier we can call it anything else than a maybe life or anno ..etc  and at the arg that the return ref to it -> so this will tell rust clearly that the refturn ref is pointing at or it's kind of the same thing as the first ref 
fn necxt_language<'a>(languages :& 'a [String], current: &str) -> & 'a str  // if we left the return type as &str it will give a error that we are missing a lifttime specifier
{
    let mut found = false;
    for  lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }
    // return the default value (the vary last language in our vector) if we didn't found the language
    languages.last().unwrap() // last return option enum but for now we willl make a big assumption that we always going to be somthing inside of language 
    // if we get none so unwrap will cause a panic!
}
//return last element inside the the vector 
//there are 1 or 2 corner cases where we can remove or not have to write out the annotation at all 
//when we recieve ref and return ref rust assume the the return is tied to this arg -> you can optionally add in lifetime annotation 
fn last_language (languages :&[String]) -> &str
{
    languages.last().unwrap()
} 
// the issue here is the return always going to point at the first atg or the second becuase the func depend on the actual input 
// so we add the the lifetime annotation to the both arg 
fn longest_language <'a>(language1 :&'a str, language2: &'a str) -> &'a str
{
    if language1.len() > language2.len()
    {
        language1
    }
    else{
        language2
    }
}
fn main() {
   let languages = vec![
    String::from("rust"),
    String::from("go"),
    String::from("typescript"),
    String::from("bash"),
   ];

   let result = necxt_language(&languages, "go");
   println!("{:#?}", result);

   let last_lang = last_language(&languages);
   println!("{:#?}", last_lang);

   let longest_lang = longest_language("typescript", "bash");
   println!("{:#?}", longest_lang);

}
