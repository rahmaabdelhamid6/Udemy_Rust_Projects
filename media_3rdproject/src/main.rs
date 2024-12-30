// enum vs struct 
// enum -> each thing you're modeling have the sme methods
// struct -> each thing have some same but some different methods

mod content;
use content::{media::Media, catalog::Catalog};
// fn print_media(media: Media)
// {
//     println!("{:#?}", media);
// }

fn main() {
    let audio_book = Media::AudioBook { 
        title: String::from("An AudioBook") };

    let bad_book = Media::Book { 
            title: String::from("Bad Book"),
             author: String::from("Bad Author") };

    let good_movie = Media::Movie { 
            title: String::from("Good Movie"), 
            director: String::from("Good Director") };

    let podcast = Media::Podcast { episode_number: (10) };
    let placeolder = Media::Placeholder;

        //    println!("{:#?}", audio_book.description());
        //    println!("{:#?}", good_movie.description());
        //    println!("{:#?}", bad_book.description());

    // print_media(audio_book);
    // print_media(good_movie);
    // print_media(bad_book);
    let mut catalog = Catalog::new();
    catalog.add(audio_book);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeolder);
    // println!("{:#?}", catalog);
    // println!("{:#?}", catalog.items.get(100)); // get is an implemented method belong to every vector, // try to 0 it will gve some and the 1st item 
    // try to put 100 it get none 
    //rust doesn't have null ,nill or undefined in it -> it has a build in enum Option {Some (value), None} dealing with it with match statement or if let
    
    // match catalog.get_by_index(100) {
    //    Some(value) => {
    //         println!("items : {:#?}", value)
    //     }
    //     None => {
    //         println!("Nothing at that index")
    //     }
    // }

    // trying to do somthing like option 
    // let item = catalog.get_by_index(40);
    // println!("{:#?}", item);
    // or 
    // match catalog.get_by_index(0) {
    //     MightHaveAValue::TherIsAValue(value) => {
    //         println!("Item: {:#?}", value)
    //     }
    //     MightHaveAValue::NoValueAvailable =>
    //     {
    //         println!("no value here")
    //     }
        
    // }

    // another ways instead of using (match with option ) , in the doc more than three but we will only demponstarte 3 of them , the three ways we will demonstrate right now are about getting access to the value inside option , so thery are assuming you have a some not a none
    let item = catalog.get_by_index(40);
    //1- unwrap
    // used for quick debugging or exampels , but it not recommended to use because it's going to panic the program very easily and crash
    // println!("{:#?}",item.unwrap()); // if we have a some unwrap is going to give you the value that belongs to some , if item is none , panics !
    //2- expect
    // the message isnide it used for debugging purpose 
    // used when we want to crash if there is no value  , ex: when we we need to use environment variable with defined value and check if ther're defined or not 
    // println!("{:#?}",item.expect("expected there to be an item here!")); // when we call it on some we will get the value inside some , if we calledit on none get a panic that feature whateever messegae put in right here
    //3- unwrap_or 
    // used when it make sense to give a fallback value or default value
    let placeholder= Media::Placeholder;
    println!("{:#?}",item.unwrap_or(&placeholder)); // if item is some return the value in some , if none return provided default value 
}
