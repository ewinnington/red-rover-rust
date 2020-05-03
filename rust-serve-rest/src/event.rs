use std::collections::HashMap;

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use actix_web::web::{Json, Path};
use actix_web::{HttpResponse};

//Importing constants
use crate::constants::APPLICATION_JSON;
use crate::response::Response;

pub type Events = Response<Event>;

#[derive(Debug, Deserialize, Serialize)]
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
    static ref HASHMAP: HashMap<String, Event> = {
        let mut m = HashMap::new();
        m.insert(Uuid::new_v4().to_string(), Event::new("First event".to_string()));
        m
    };
}

/// list 50 last event `/events`
#[get("/events")]
pub async fn list() -> HttpResponse {
    let events = Events { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(events)
}

/// create a event `/events`
#[post("/events")]
pub async fn create(event_req: Json<EventRequest>) -> HttpResponse {
    
    
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(event_req.to_event())
}

/// find a event by its id `/events/{id}`
#[get("/events/{id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
    
    let mut found_event: Option<Event> = None;

    let id = path.0.as_str(); 
    if HASHMAP.contains_key(id)
    {
        let myEvent = HASHMAP.get(id);
        found_event = myEvent;
    }

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
    //HASHMAP.remove_entry(&id); 

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}