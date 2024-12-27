use std::string;


#[derive(Debug)]
struct Account{
    id: u32,
    balance: i32,
    holder: String,
}
impl Account{
    fn new (id: u32, holder: String) -> Self{
        Account{
            id,
            holder, 
            balance: 0,
        }

    }
    fn deposit(&mut self, amount:i32) ->i32// amount could be &readonly ref or value as i32 have a copy mechanism and i can use it multiple time within the main
    {
        self.balance +=  amount;
        self.balance
        
    }
    fn withdraw(&mut self, amount:i32) -> i32{
        self.balance -= amount;
        self.balance

    }
    fn summary(&self) -> String{
        format!("{} has a balance {}", self.holder, self.balance)
    }
}
#[derive(Debug)] 
struct Bank{
    accounts: Vec<Account>,
}

impl Bank{
    fn new() -> Self{
        Bank { accounts: vec![] }
    }
    fn add_account(&mut self, account: Account) //if we left it &self kda d readonly reference for bank struct but we want to add an account to the vector of accounts so we make it &mut self
    {
        self.accounts.push(account);
    }
    fn total_balance(&self) ->i32{
        self.accounts.iter().map(|account| account.balance).sum()
    }
    fn summary(&self) ->Vec<String>{
        self.accounts
        .iter()
        .map(|account|account.summary())
        .collect::<Vec<String>> ()

    }

}
fn main(){
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Rahma"));
    let deposit_amount = 50;
    let withdraw_amount = 30;
    
    account.deposit(deposit_amount);
    account.withdraw(withdraw_amount);

    // println!(" {}",account.summary());

    bank.add_account(account);
    // println!("{:#?}", bank);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
  

}


// fn print_account(account: Account){
//     println!("{:#?}", account);
// }
// fn print_holder(holder:String){
//     println!("{}", holder);

// }

// fn print_account2(account: Account) -> Account{
//     println!("{:#?}", account); //take the ownership of the account value then return the ownership again ti account binding soi can use the function multiple times for the same binding
//     account
// }

// fn print_account3(account: &Account){
//     println!("{:#?}", account);
// }
// // &mut indicate that the function needs to be called with mutable ref 
// fn change_account(account:&mut Account){
//     account.balance =10;
// }
// // fn main() {
    
// //     let bank = Bank::new();
//     // println!(" {:#?}", bank);
    
//     // let  account = Account::new(1,  String::from("Rahma")); //to make an actrual sytring not slice Strong::from()
//     // let account_ref = &account; // d kda b point at account oignal value ,readable reference allow us to look at a value without moving it 
//     // let account_ref2 = &account; // can make a multiple ref to the smae binding and use them all
//     // let other_account = account; //can't move a value while a ref to the value exists.

//     // print_account3(account_ref);
//     // account_ref.holder = 10; // readable ref is readonly event if  i add mut before it it's only a reference and doeasn't have the ability to change or to assign using it only view data 
//     // println!("{}", account_ref.holder);  // to vew data only
//     //or 
//     // print_account3(&account); //no need to make a new binding to hold the ref 

//     // println!("{:#?}", account);
//     //counter part writeable reference ---> you can make them onlu when there are no readonly ref currently in use 
//     //writeable ref can use to change or read a data  without moving it--> used most of ti,es when i need a helper function to change some input valiue 
//     //lw hawelt a3ml readonly ref aw writeable ref tani m3 el elly mawgud f el func haydini error 
//     // let mut account = Account::new(1,  String::from("Rahma"));
//     // let account_ref = &mut account;
//     // change_account(&mut account); //&mut used to create a mutable ref 
//     // account.balance = 10;
    
//     // println!("{:#?}", account);
//     // println!("{:#?}", account_ref);

//     //you can't mutate a value through the owner when any ref (mutable, immutable to the value exists)
//     // change_account(&mut account);
//     // change_account(account_ref);
   
//     // let mut account = Account::new(1,  String::from("Rahma"));
//     // account = print_account2(account);
//     // print_account2(account); //make the account binding mut to reassign in it again 

//     // print_holder(account.holder);
//     // print_account(account);

//     // let lost_of_accounts = vec![account];
    
//     // print_account(account);
//     // println!("{:#?}", account.holder);

//     // print_account(account); //use of moved value error 

//     // let accounts = bank.accounts;
//     // println!("{:#?}", bank.accounts);
    
//     // let other_bank = bank;
//     // println!("{:#?}", bank);


// // }


// fn print_balance(account: &Account) {
//     println!("{}", account.balance);
// }
 
// fn main() {
//     let mut bank = Bank::new();
//     let account = Account::new(1, String::from("me"));
 
//     // let account_ref = &account;
    
//     print_balance(&account);
    
//     bank.accounts.push(account);
// }
