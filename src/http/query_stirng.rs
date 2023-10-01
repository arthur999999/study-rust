use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'inBufferStream> {
    data: HashMap<& 'inBufferStream str, Value<'inBufferStream>>
}

#[derive(Debug)]
pub enum Value<'inBufferStream> {
    Single(& 'inBufferStream str),
    Multiple(Vec<& 'inBufferStream str>)
}

impl<'inBufferStream> QueryString<'inBufferStream> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'inBufferStream> From<&'inBufferStream str> for QueryString<'inBufferStream> {
    fn from(s: &'inBufferStream str) -> Self {
            
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find("=") {
                key = &sub_str[..i];
                val = &sub_str[i+1..];

            }

            data.entry(key).and_modify(|existing_map | match existing_map{
                Value::Single(old_val) => {
                    *existing_map = Value::Multiple(vec![old_val, val]);
                }
                Value::Multiple(vec)=> vec.push(val),
            }).or_insert(Value::Single(val));
        }

        

       QueryString { data: data }
    }
} 