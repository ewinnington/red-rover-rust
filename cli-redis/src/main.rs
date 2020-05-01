extern crate redis;
use redis::Commands;
//https://docs.rs/redis/0.15.1/redis/

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;

    let _ : () = con.set("Key1", "Hello World!")?;

    //Declaring the value return type 
    let key_value : String = con.get("Key1")?;

    println!("{:?}", key_value);
    Ok(())
}


fn main() {
    let _  = do_something(); 
}
