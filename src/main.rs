use std::fs;
use rand::Rng;

struct Quote {
    quote : String,
    quotee : String,
    date : Option<String>,
    work : Option<String>,
    author : Option<String>,
}

fn main() {

    let mut quotes : Vec<Quote> = Vec::new();

    if let Ok(data) = fs::read_to_string("res/quotes.txt") {
        let lines = data.lines();
        for line in lines {
            let parts : Vec<&str> = line.split("-").collect();
            quotes.push(Quote {
                quote: parts[0].to_owned(),
                quotee: parts[1].to_owned(),
                date: None,
                work: None,
                author: None
            }) 
        }
    }
    
    let r = rand::thread_rng().gen_range(0..quotes.len());
    let output = format!("{}-{}", quotes[r].quote.clone(), quotes[r].quotee.clone());
    println!("{}", output);
    
}


fn add ()
{
    
}
