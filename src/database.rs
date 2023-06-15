use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::types::AttributeValue::S;
use aws_sdk_dynamodb::Error;
use rocket::State;

pub async fn store_short_hash_in_dynamo(
    short_hash: String,
    full_url: String,
    dynamo_client: &State<aws_sdk_dynamodb::Client>,
) -> Result<(), Error> {
    let dynamo_table_name = "url_shortener";

    let request = dynamo_client
        .put_item()
        .table_name(dynamo_table_name)
        .item(
            "short_url",
            AttributeValue::S(String::from(short_hash.to_string())),
        )
        .item(
            "original_url",
            AttributeValue::S(String::from(full_url.to_string())),
        );

    request.send().await?;

    Ok(())
}

pub async fn retrieve_shortened_url_from_dynamo(
    short_hash: String,
    dynamo_client: &State<aws_sdk_dynamodb::Client>,
) -> Result<String, Error> {
    let dynamo_table_name = "url_shortener";

    let dynamo_data = dynamo_client
        .get_item()
        .table_name(dynamo_table_name)
        .key(
            "short_url",
            AttributeValue::S(String::from(short_hash.to_string())),
        )
        .send()
        .await?;

    let s_url = dynamo_data
        .item()
        .as_ref()
        .unwrap()
        .get("original_url")
        .unwrap();

    if let S(url) = s_url {
        return Ok(url.clone());
    }

    Ok("".to_string())
}
