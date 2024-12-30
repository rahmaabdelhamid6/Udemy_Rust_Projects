#[derive(Debug)]
pub enum Media {
    Book{title: String, author : String}, //variants 
    Movie{title: String, director : String},
    AudioBook{title :String},
    Podcast{episode_number :u32}, // or   Podcast(u32) -> same thing
    Placeholder // doesn't have any data type tied to it 
}
impl Media {
    pub fn description(&self) -> String{
        //  String::from("Media Description")
        //can't access the title property before deciding what is the media (book, audio, movie)?
        //so there's 2 ways to do that 
        // //1- it's a verbose way --> if (we have a book){ format!("book : {} {}", title , author)} else if () etc....else { string::format("media descripton")}
        // //if we have a book --> then give me access o title and author inside of this branch
        // if let Media::Book { title , author } = self {
        //     format!("Book:{} {}", title,author)
        // }else if let Media::Movie{ title, director } = self {
        //     format!("Movie: {} {}", title,director)
        // }else if let Media::AudioBook { title } = self {
        //     format!("Audio Book: {}", title)
        // }else {
        //     String::from("Media Description ")
        // }
        //2- patter match statement -->it's slightly cleaner and easier to understand used alot with enum to handle enum when you need to figure out what type the enum is
        match self // it must cover all the variant inside the enum 
        {
            Media::Book { title, author } => //must list all the property
            {
                format!("Book:{} {}", title,author) //no ; beacause we want to use implicit return to automatically return that (the impl descripttion function return string)
            } 
            Media::Movie { title, director }  => {
                format!("Movie: {} {}", title,director)
            }
            Media::AudioBook { title } => {
                format!("Audio Book: {}", title) 
            }  
            Media::Podcast { episode_number} =>{
                format!("Podcast: {}", episode_number)    
            }
            Media::Placeholder =>{
                format!("Placeholder:")    
            }
         }

    }
}
