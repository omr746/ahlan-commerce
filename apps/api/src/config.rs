use std::env;

pub const HOST:&str="APP_HOST";
pub const PORT:&str="APP_PORT";
pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
pub struct Config{
   pub host:String,
    pub port:u16,
     pub redis_url: String, 
    pub database_url:String
}
impl Config{
   pub fn new()->Self
{
    let host=env::var(HOST).unwrap_or_else(|_| "0.0.0.0".to_string());
    let port=env::var(PORT).ok().and_then(|p|p.parse().ok()).unwrap_or(3000);
     let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| {
            "redis://127.0.0.1:6379".to_string()
        });
     let database_url = env::var(ENV_DATABASE_URL).unwrap_or_else(|_| {
            "postgres://postgres:132456@127.0.0.1:5432/ahlan-commerce".to_string()
        });
        Self { host, port, redis_url, database_url }

}
pub fn addr(&self)->String{
    format!("{}:{}",self.host,self.port)
}
}