use ureq::json;

fn main() {

// sync post request of some json.
    let resp = ureq::post("http://localhost:9090/events")
    .set("Content-Type", "application/json")
    .send_json(json!({
        "name": "Event posted from Rest client"
    }));

// .ok() tells if response is 200-299.
    if resp.ok() {
        let id : String = resp.into_json().unwrap()["id"].to_string(); 
        println!("Event created: {}", id);

        let get = ureq::get("http://localhost:9090/events").call().into_json();
        println!("Events: {:?}", get);

        let del = ureq::delete(&["http://localhost:9090/events/", &id].concat()).call(); 
        if del.ok() {
            println!("Event deleted: {}", id); 
        }
        
        let get = ureq::get("http://localhost:9090/events").call().into_json();
        println!("Events: {:?}", get);
    }
    else
    {
        println!("{:?}", resp);
    }
}
