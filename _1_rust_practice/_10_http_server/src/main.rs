use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::TcpListener;

mod model;
mod handle;
mod api;
mod routes;

use model::Student;
use handle::load_students;
use routes::create_routes;

pub type SharedState = Arc<RwLock<Vec<Student>>>;

#[tokio::main]
async fn main() {
   
    let students = load_students();

    let state: SharedState = Arc::new(RwLock::new(students));

    let app = create_routes(state);

    let listener = TcpListener::bind("127.0.0.1:4500")
        .await
        .unwrap();

    println!("Server running at http://127.0.0.1:4500");

    axum::serve(listener, app).await.unwrap();
}
