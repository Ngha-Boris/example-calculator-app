use poem_openapi::{Object, OpenApiService, Tags};
use tokio::sync::Mutex;

pub mod adder;
pub mod subtractor;
pub mod counter;

use adder::Adder;
use subtractor::Subtractor;
use counter::Counter;

#[derive(Tags)]
pub enum ServiceTags {
  Adder,
  Subtractor,
  Counter,
}

#[derive(serde::Deserialize, Object)]
struct CalculatorRequest {
  a: i32,
  b: i32,
}

#[derive(serde::Serialize, Object)]
struct CalculatorResponse {
  result: i32,
}

pub fn make_service() -> OpenApiService<(Adder, Subtractor, Counter), ()> {
  OpenApiService::new((Adder, Subtractor, Counter(Mutex::new(0))), "Calculator", "1.0")
}
