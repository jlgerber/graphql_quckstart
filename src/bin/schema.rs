use juniper::{
    //EmptyMutation, 
    EmptySubscription,
    IntrospectionFormat,
};
use serde_json;

use quickstart::*;

fn main() {
    let ctx = Database::new();
    let (res, _errors) = juniper::introspect(
        &Schema::new(Query, Mutations{}, EmptySubscription::new()),
        &ctx,
        IntrospectionFormat::default(),
    ).unwrap();

    let json_result = serde_json::to_string_pretty(&res).expect("unable to conver schema to json");
    println!("{}", json_result);

}