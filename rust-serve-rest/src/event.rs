use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use actix_web::web::{Json, Path};
use actix_web::{HttpResponse};

//Importing constants
use crate::constants::APPLICATION_JSON;
use crate::response::Response;

pub type Events = Response<Event>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String
}

impl Event {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            name
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventRequest {
    pub name: Option<String>,
}

impl EventRequest {
    pub fn to_event(&self) -> Option<Event> {
        match &self.name {
            Some(name) => Some(Event::new(name.to_string())),
            None => None,
        }
    }
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, Event>> = {
        let m = Mutex::new(HashMap::new());
        let e = Event::new("First event".to_string()); 
        m.lock().unwrap().insert(e.id.to_string(), e);
        m
    };
}

/// list 50 last event `/events`
#[get("/events")]
pub async fn list() -> HttpResponse {
    //let events = Events { results: vec![] };

    let events = Events { results: HASHMAP.lock().unwrap().values().cloned().collect() };
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(events)
}

/// create a event `/events`
#[post("/events")]
pub async fn create(event_req: Json<EventRequest>) -> HttpResponse {
    
    let e = event_req.to_event().unwrap();
    HASHMAP.lock().unwrap().insert(e.id.to_string(), e);
    
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(event_req.to_event())
}

/// find a event by its id `/events/{id}`
#[get("/events/{id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
    let id = path.0.as_str(); 

    let map = HASHMAP.lock().unwrap(); 
    let found_event = map.get(id);
  
    match found_event {
        Some(event) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(event),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a event by its id `/tweets/{id}`
#[delete("/events/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
    
    let id = path.0.as_str(); 
    let mut map = HASHMAP.lock().unwrap(); 
    map.remove_entry(id);

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}