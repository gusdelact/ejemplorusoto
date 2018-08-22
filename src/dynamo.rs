extern crate rusoto_core;
extern crate rusoto_dynamodb;

use std::default::Default;
use std::collections::HashMap;

use self::rusoto_core::Region;
use self::rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use self::rusoto_dynamodb::{CreateTableInput,KeySchemaElement,AttributeDefinition, PutItemInput, AttributeValue};

pub fn dynamo() {
  let client = DynamoDbClient::new(Region::UsEast1);
  let list_tables_input: ListTablesInput = Default::default();

  match client.list_tables(list_tables_input).sync() {
    Ok(output) => {
      match output.table_names {
        Some(table_name_list) => {
          println!("Tables in database:");

          for table_name in table_name_list {
            println!("{}", table_name);
          }
        }
        None => println!("No tables in database!"),
      }
    }
    Err(error) => {
      println!("Error: {:?}", error);
    }
  }

//crear una Tabla
    let mut table_input: CreateTableInput = Default::default();
    table_input.table_name="rusty_table".to_string();
    let key : KeySchemaElement = KeySchemaElement { attribute_name: "index".to_string(), key_type:"HASH".to_string()};
    table_input.key_schema.push(key);
    let attrKey : AttributeDefinition = AttributeDefinition{ attribute_name: "index".to_string(), attribute_type: "N".to_string()};
    table_input.attribute_definitions.push(attrKey);
    table_input.provisioned_throughput.read_capacity_units=5;
    table_input.provisioned_throughput.write_capacity_units=5;
    
    match client.create_table(table_input).sync() {
      Ok(table_output) => {
         println!("creada");
         let mut item : PutItemInput =  Default::default();
         item.table_name= "rusty_table".to_string();
         let mut map = HashMap::new();
         let mut valorIndex : AttributeValue = Default::default();
         let mut valorCampo0 : AttributeValue = Default::default();
         valorIndex.n=Some("123".to_string());
         valorCampo0.s=Some("unvalor".to_string());
         map.insert("index".to_string(),valorIndex);
         map.insert("campo0".to_string(),valorCampo0);
         item.item = map;
         client.put_item(item);
      }
     Err(error) => {
      println!("Error: {:?}", error);
     }
      
   }
}
