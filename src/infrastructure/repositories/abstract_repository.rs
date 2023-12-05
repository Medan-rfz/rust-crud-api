use async_trait::async_trait;

#[async_trait]
pub trait Repository<T> {
    async fn get_by_id(&self, id: i32) -> Option<T>;
    async fn add(&mut self, item: T) -> bool;
    async fn update(&mut self, id: i32, new_item: T) -> bool;
    async fn remove(&mut self, id: i32) -> bool;
}
