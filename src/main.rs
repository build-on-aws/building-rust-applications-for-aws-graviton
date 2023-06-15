use aws_sdk_dynamodb::Client;
use rocket::http::Status;
use rocket::State;
mod database;
mod shortener;

use database::retrieve_shortened_url_from_dynamo;
use database::store_short_hash_in_dynamo;
use shortener::get_url_short_id;

#[macro_use]
extern crate rocket;

#[get("/")]
fn default_response() -> &'static str {
    "Hello, world!"
}

#[get("/get_full_url", data = "<short_url>")]
async fn get_full_url(
    short_url: String,
    dynamo_client: &State<aws_sdk_dynamodb::Client>,
) -> Result<String, Status> {
    let long_url = retrieve_shortened_url_from_dynamo(short_url, dynamo_client)
        .await
        .expect("failed to get key");
    Ok(format!("Your full URL is {long_url}"))
}

#[post("/shorten_url", data = "<url>")]
async fn shorten_url(
    url: String,
    dynamo_client: &State<aws_sdk_dynamodb::Client>,
) -> Result<String, Status> {
    let shortened_url: String = get_url_short_id();
    store_short_hash_in_dynamo(shortened_url.to_string(), url.to_string(), dynamo_client)
        .await
        .expect("Failed to write data.");

    Ok(format!("https://myservice.localhost/{shortened_url}"))
}

#[launch]
async fn rocket() -> _ {
    let aws_client_config = aws_config::load_from_env().await;
    let dynamo_client = Client::new(&aws_client_config);

    rocket::build()
        .mount("/", routes![default_response, get_full_url, shorten_url])
        .manage(dynamo_client.clone())
}
