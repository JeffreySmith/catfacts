use serde::{Deserialize,Serialize};
use argh::FromArgs;

#[derive(FromArgs,Debug)]
/// Get a fun cat fact
struct Args{
    ///max length of cat fact
    #[argh(option)]
    length:Option<i32>,
}

#[derive(Serialize,Deserialize,Debug)]
struct Fact{
    fact:String,
    length:i32,
}
#[tokio::test]
async fn test_fact(){
    let client = reqwest::Client::new();
    let fact = get_fact(&client,Some(20)).await;
    match fact{
        Ok(cat_fact)=>assert_eq!("Cats have 3 eyelids.",cat_fact.fact),
        Err(e)=>panic!("Error occured: {}",e),
    }
}
async fn get_fact(client:&reqwest::Client,length:Option<i32>) -> Result<Fact,serde_json::Error>
{
    let url = match length {
        Some(len) => format!("https://catfact.ninja/fact?max_length={}",len),
        _ => "https://catfact.ninja/fact".to_string()
    };
    
    let response = client
        .get(url)
        .send()
        .await;

    let mut input:String = String::new();
    match response{
        Result::Ok(req) => {
            if let Result::Ok(txt)=req.text().await{
                input = txt;
            }
        },
        Result::Err(e) => println!("Error: {}",e),
    };
    serde_json::from_str::<Fact>(&input[..])
    
}
#[tokio::main]
async fn main() {
    let arg:Args = argh::from_env();
    let client = reqwest::Client::new();

    let my_fact = get_fact(&client,arg.length).await;
    match my_fact{
        Ok(f)=>println!("{}",f.fact),
        Err(e)=>println!("{}",e),
    }
}
