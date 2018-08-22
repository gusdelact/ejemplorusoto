extern crate rusoto_core;
extern crate rusoto_lex_runtime;

use std::default::Default;
use std::collections::HashMap;
use self::rusoto_core::Region;
use self::rusoto_lex_runtime::{LexRuntimeClient,LexRuntime};
use self::rusoto_lex_runtime::{PostTextRequest,PostTextResponse,PostTextError};
pub fn dialogo() {
   let client = LexRuntimeClient::new(Region::UsEast1);
   let mut text_req:PostTextRequest = Default::default();
   text_req.bot_name="BookTrip".to_string();
   text_req.bot_alias="dev".to_string();
   text_req.user_id="rusoto".to_string();
   text_req.input_text="book a car".to_string();
   match client.post_text(text_req).sync() {
      Ok(output) => {
        println!("dialogo");
      println!("{:?}",output.message.unwrap());
        println!("{:?}",output.dialog_state.unwrap()); 
      }
      Err(error) => {
       println!("Error");
    //   println!("Error: {:?}", error);
      }
   }
}
