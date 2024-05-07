use csv::Reader;
use serde::{Serialize,Deserialize};
use anyhow::Result;
use std::fs;

use crate::opts::OutputFormat;

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="PascalCase")]
struct Player{
    
    name:String,
    position:String,
    #[serde(rename="DOB")]
    dob:String,
    nationality:String,
    #[serde(rename="Kit Number")]
    kit:u8,

}
//convert csv to json
pub fn process_csv(input:&str,output:&str,format:OutputFormat)->Result<()>{
    let mut reader=Reader::from_path(input)?;
            let headers=reader.headers()?.clone();
            let mut ret=Vec::with_capacity(128);
            for record in reader.records(){
                let record=record?;
                let json_value=headers.iter().zip(record.iter()).collect::<serde_json::Value>();
                // println!("{}",json_value);
                ret.push(json_value);
            }
            let content=match format{
                OutputFormat::Json=>serde_json::to_string_pretty(&ret)?,
                OutputFormat::Toml=>toml::to_string(&ret)?,
                OutputFormat::Yaml=>serde_yaml::to_string(&ret)?,
            };
            fs::write(output,content)?;
            Ok(())
}