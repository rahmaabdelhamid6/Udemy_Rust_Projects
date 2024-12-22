//crate == package --> some bit of code we install in our project 
// stdandad libaray doc.rust-lang.org/std included with every project without any additional install 
//external crate have to be install into the project with command --> (cargo add rand) , documentation --> docs.rs , crates.io


//if we're using internal module so we need to mention it at the beginging of the file like (mod games);), then inside the main we call it games::Deck:;new();
// but if we use externel crate or module the we had add it  w call cratename::rootmodulename::submodulename::funcname
//what if we gonna use a multiple function from external crate ? we can write (use cratename::{modulename, modulename::submodule,etc}) at the top of the file and access it direct in the main code funcname();

use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)] //debug trait--> set of functions 
struct Deck{
    cards: Vec<String>,  
}

//inherent implementation 
impl Deck {
    //associated function , when we call it Struct Name::function name
    //function not tied to specific instance
    fn new() -> Self //return type annotation , reference to type mentioned in the parent impl block
    {
    //we call them bindings not variabelsin rust beacause they are immutable (forbidding two different operation 1- changing the value , 2-reassign the value )
    let suits = ["hearts", "spades", "diamond"]; 
    let values = ["ace", "two", "three"];
    let mut cards = vec![];
    for suit in suits{
        for value in values{
            let card = format!("{} of {}", value, suit); // join together strings in some intelligent fashion 
            cards.push(card);
        }
    }

    // let deck = Deck{cards};  // cards:: Vec::new()---> same thing  , Deck {cards: cards} when the type have the same as bindings we right the name only
    // return deck;
    //or
    // return Deck{cards};  // cards:: Vec::new()---> same thing  , Deck {cards: cards} when the type have the same as bindings we right the name only
    //or 
    Deck{cards} // called implicit return --> automatically return last executed line inside the function 
    }
    //method when we call it struct name.method name and the argument in it is &self
    //and it operate on very specfic instance or single copy of a struct when you need to read or change data from a specific instance
    //to randomize the order of cards
    fn shuffle(&mut self)// self is reference to instance we will make of the struct so if the instance is mut the reference also must be mut
    {
        let mut rng = thread_rng(); //create a new random number generator 
        self.cards.shuffle(&mut rng); 
    }
    fn deal (&mut self, num_card:usize)  -> Vec<String>{

        self.cards.split_off(self.cards.len() - num_card)

    }
}
fn main() 
{
        let mut deck = Deck::new(); //we made it mut because the shuffle func gonna do a change some data contained inside it(nested inside that vector)  
        deck.shuffle();
        print!("Heres your deck: {:#?}", deck); //{} is a place holder == :{deck}, {:?}--> debug formater forpriniting string , {:#?} --> # is callad pound and it add print nicely format fashion 

        let cards = deck.deal(3);
        println!("Heres your hand : {:#?} ", cards);
        print!("Heres your deck: {:#?}", deck);
}
