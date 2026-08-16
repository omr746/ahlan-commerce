use std::env;

pub const HOST:&str="APP_HOST";
pub const PORT:&str="APP_PORT";

pub struct Config{
    host:String,
    port:u16
}
impl Config{
   pub fn new()->Self
{
    let host=env::var(HOST).unwrap_or_else(|_| "0.0.0.0".to_string());
    let port=env::var(PORT).ok().and_then(|p|p.parse().ok()).unwrap_or(3000);
    Self{
        host,
        port
    }

}
pub fn addr(&self)->String{
    format!("{}:{}",self.host,self.port)
}
}