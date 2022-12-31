mod generate_table;

use std::{error::Error, time::{Duration}};
use futures::prelude::*;
use ssdp_client::SearchTarget;
use crate::generate_table::adding_row;

extern crate colour;
extern crate prettytable;
extern crate chrono;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    let search_target = SearchTarget::RootDevice;
    println!("Loading...");

    let mut responses = ssdp_client::search(&search_target, Duration::from_secs(3), 2).await?;
    let mut table = generate_table::creating_table();

    // Loop over results
    while let Some(response) = responses.next().await {
        let usn = String::from(response.as_ref().unwrap().usn());
        let server = String::from(response.as_ref().unwrap().server());
        let location = String::from(response.as_ref().unwrap().location());
        // Adding data into table
        table = adding_row(table, &*usn,&*location, &*server)
    }
    table.printstd();

    Ok(())
}
