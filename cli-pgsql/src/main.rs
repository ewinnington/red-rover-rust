extern crate postgres;
use postgres::{Client, NoTls};


fn main() {
    let mut client = Client::connect("host=localhost port=5432 user=docker password=XdccDa85_JK dbname=docker sslmode=disable", NoTls).unwrap();
    println!("Connected");
    let _ = client.batch_execute("CREATE TABLE currencies(id SERIAL PRIMARY KEY, name VARCHAR(3))");


    let eur = "EUR";
    let _ = client.execute("INSERT INTO currencies (name) VALUES ($1)", &[&eur]);
    
    let statement : postgres::Statement = client.prepare("INSERT INTO currencies (name) VALUES ($1)").unwrap(); 
    let _ = client.query(&statement,&[&"CHF"]); 

    for row in client.query("SELECT id, name FROM currencies", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);

        println!("Currencies: {} {}", id, name);
    }
}
