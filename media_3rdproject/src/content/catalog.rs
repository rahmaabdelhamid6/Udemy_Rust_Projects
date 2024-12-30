use super::media::Media;

#[derive(Debug)]
pub struct  Catalog{
    items: Vec<Media>
}
impl Catalog {
    pub fn new() -> Self{
        Catalog {items :vec![]}
    }
    pub fn add (&mut self, media: Media){
        self.items.push(media);  // media is only a value because we want to take the ownership of media 
    }

    pub fn get_by_index(&self ,index :usize) -> Option<&Media>{
        if self.items.len() > index {
        Some( &self.items[index])// if we don't add the & that mean we're going to move this item outside of this vector , but we need to return a reference to this item 
     } else {
       None
     }
        // what if i'm trying to access something out of bounds of the vector
    }

    // fn get_by_index(&self ,index :usize) -> MightHaveAValue{
    //     if self.items.len() > index {
    //    MightHaveAValue::TherIsAValue( &self.items[index])// if we don't add the & that mean we're going to move this item outside of this vector , but we need to return a reference to this item 
    //  } else {
    //     MightHaveAValue::NoValueAvailable
    //  }
    //     // what if i'm trying to access something out of bounds of the vector
    // }
}
// we trying to do the Option enum built it in rust by ourselves
// #[derive(Debug)]
// enum  MightHaveAValue<'a> //this is <'a> called a life ti,e annotation and will be discussed letter 
//  {
//     TherIsAValue(&'a Media),
//     NoValueAvailable

// }
