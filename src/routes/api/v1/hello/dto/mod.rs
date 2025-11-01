use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct HelloRequest {
    #[validate(length(min = 2, max = 50))]
    pub name: String,
}
