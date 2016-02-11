use iron::prelude::*;
use iron::status;

use std::string::String;

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;

use bson::{Bson, Document};
use bson::oid::ObjectId;

use rustc_serialize::json::{Json, ToJson};

pub fn get_all(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, fetch_mongo_coll())))
}

// returns all the documents of the collection
// as properly formatted JSON
fn fetch_mongo_coll() -> String {
    // connect to the default monogo instance
    // TODO : make ThreadedClient
    // TODO : see about pulling connection details
    // from config
    let client = Client::connect("localhost", 27017)
        .ok()
        .expect("Failed to init mongo client.");

    // connet to the db and collection
    // TODO :  make ThreadedDatabase
    let coll = client.db("OrganicForge").collection("gmo");

    // get all of the collection as a mongo::Cursor
    // NOTE: http://mongodb-labs.github.io/mongo-rust-driver-prototype/mongodb/cursor/struct.Cursor.html
    let cursor = coll.find(None, None).unwrap();

    // call map on the cursor
    // pass map the closure to be executed on each member of the cursor
    // in the closure parse the Bson::Document to json and format as string
    // collect the results of the closure
    // TODO : add error handling
    let collec: Vec<String> = cursor.map(|x| {
                                        match get_data_json(x) {
                                            Ok(data) => format!("{}", data),
                                            Err(e) => format!("{}", e)
                                        }
                                    })
                                    .collect();

    // return the collection as a JSON string
    format!("{}", collec.to_json())
}

// Parses the result BSON document to json or
// returns the error as a string
fn get_data_json(result: MongoResult<Document>) -> Result<Json, String> {
    match result {
        Ok(doc) => Ok(Bson::Document(doc).to_json()),
        Err(e) => Err(format!("{}", e))
    }
}
