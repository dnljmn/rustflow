use std::{collections::HashMap, fmt, string};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline{
    pub name: string::String,
    pub input: Vec<Input>
}

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "");
        
        write!(f, "\nPipeline[{}]:", self.name)
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Input {
    Tail {
        name: String,
        path: Vec<String>,
    },
    File {
        name: String,
        zulu: Vec<String>,
    },
    // Add more input types as needed
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DataInput{
    Tail(TailInput)
}

impl Pipeline{

}



#[derive(Debug, Serialize, Deserialize)]
pub struct TailInput{

}

