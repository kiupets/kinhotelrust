use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::rented_model::Rented;
use crate::models::user_model::User;
use actix::Addr;
use actix_web::web::Data;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
};
use serde_json::to_value;

use crate::websocket::{MessageToClient, Server};

pub struct MongoRepo {
    col: Collection<User>,
    col_rented: Collection<Rented>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("kinhotelrust");
        let col: Collection<User> = db.collection("User");
        let col_rented: Collection<Rented> = db.collection("Rented");

        MongoRepo { col, col_rented }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            email: new_user.email,
            phone: new_user.phone,
            room: new_user.room,
            start: new_user.start,
            end: new_user.end,
            interval_rented_array: new_user.interval_rented_array,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn create_rented(
        &self,
        new_rented: Rented,
        websocket_srv: Data<Addr<Server>>,
    ) -> Result<InsertOneResult, Error> {
        let new_doc = Rented {
            id: None,
            interval_rented_array: new_rented.interval_rented_array,
        };
        let rented = self
            .col_rented
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating rented");
        if let Ok(new_doc) = to_value(new_doc.clone()) {
            let msg = MessageToClient::new("new_doc", new_doc);
            websocket_srv.do_send(msg);
        }
        Ok(rented)
    }

    pub fn get_rented(&self, id: &String) -> Result<Rented, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let rented_detail = self
            .col_rented
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(rented_detail.unwrap())
    }
}
