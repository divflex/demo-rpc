
#[tarpc::service]
pub trait Hello {
    async fn say_hi() -> String;
}
