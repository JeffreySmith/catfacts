#![warn(clippy::all,clippy::pedantic)]
use serde::{Deserialize,Serialize};
use argh::FromArgs;

#[derive(FromArgs)]
/// Get a fun cat fact
struct Args{
    ///max number of characters in the cat fact
    #[argh(option)]
    length:Option<i32>,
    ///number of facts you want
    #[argh(option)]
    facts:Option<i32>
}

#[derive(Serialize,Deserialize,Debug)]
struct Fact{
    fact:String,
    length:i32,
}

#[tokio::test]
async fn test_fact(){
    let client = reqwest::Client::new();
    //There's only one 20 or fewer character fact
    let fact = get_fact(&client,Some(20)).await;
    match fact{
        Ok(cat_fact)=>assert_eq!("Cats have 3 eyelids.",cat_fact.fact),
        Err(e)=>panic!("Error occured: {}",e),
    }
}

//Using anyhow::Result, we can abstract away some of the annoying multiple error types with serde_json and reqwest
async fn get_fact(client:&reqwest::Client,length:Option<i32>) -> anyhow::Result<Fact>
{
    let url = match length {
        Some(len) => format!("https://catfact.ninja/fact?max_length={len}"),
        _ => "https://catfact.ninja/fact".to_string()
    };
    
    let response = client
        .get(url)
        .send()
        .await;

    let input:anyhow::Result<String> = match response{
        Result::Ok(req) => {
            match req.text().await {
                Result::Ok(txt) => Ok(txt),
                Result::Err(e) => Err(e.into()),
            }
        },
        Result::Err(e) => Err(e.into())
    };
    
    match input{
        Ok(i) => Ok(serde_json::from_str::<Fact>(&i[..])?),
        Err(e) => Err(e),
    }
}

#[tokio::main]
async fn main() {
    let arg:Args = argh::from_env();
    let client = reqwest::Client::new();
    let number_of_facts = arg.facts.unwrap_or(1);
    
    for i in 1..=number_of_facts{
        if number_of_facts > 1 {
            print!("Fact {i}:\t");
        }

        let fact = get_fact(&client,arg.length).await;
        match fact{
            Ok(f)=>println!("{}",f.fact),
            Err(error)=> println!("Error: \n{error}"),
        }       
    }
}
