use async_trait::async_trait;
use std::error::Error as StdError;
use crate::domain::entities::user::User;
use crate::settings::PG_CONNECTION_STRING;
use super::abstract_repository::Repository;
use tokio_postgres::{NoTls, Client};

pub struct UserRepository {
    pub client: Client,
}

impl UserRepository {
    pub async fn new() -> Result<Self, Box<dyn StdError>> {
        let res = tokio_postgres::connect(PG_CONNECTION_STRING, NoTls).await;

        if res.is_ok() {
            let (client, connection) = res.unwrap();
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Connection error: {}", e);
                }
            });
            
            return Ok(Self { 
                client: client
            });
        }

        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Some error message")));
    }
}

#[async_trait]
impl Repository<User> for UserRepository {
    async fn get_by_id(&self, id: i32) -> Option<User> {
        let query: &str = "SELECT * FROM \"users\" WHERE id=$1";
        let res = self.client.query(query, &[&id]).await;

        if res.is_ok() {
            let rows = res.unwrap();
            if rows.is_empty() { 
                return Option::None; 
            }
            return Option::Some(User {
                id: rows[0].get("id"),
                name: rows[0].get("name"),
                age: rows[0].get("age"),
            });
        }

        return Option::None;
    }

    async fn add(&mut self, item: User) -> bool {
        let query: &str = "INSERT INTO users (name, age) VALUES ($1, $2)";
        let res = self.client.execute(query, &[&item.name, &item.age]).await;
        return res.is_ok();
    }

    async fn update(&mut self, id: i32, new_item: User) -> bool {
        let query: &str = "UPDATE \"users\" SET name=$2, age=$3 WHERE id=$1";
        let res = self.client.execute(query, &[&id, &new_item.name, &new_item.age]).await;
        return res.is_ok();
    }

    async fn remove(&mut self, id: i32) -> bool {
        let query: &str = "DELETE FROM users WHERE id=$1";
        let res = self.client.execute(query, &[&id]).await;
        return res.is_ok();
    }
}
