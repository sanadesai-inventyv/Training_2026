use axum::{
    extract::{Path, State},Json,http::StatusCode,
};
use uuid::Uuid;

use crate::{model::Student, handle::save_students, SharedState};


pub async fn get_students(State(state): State<SharedState>) -> Json<Vec<Student>> {
    let students = state.read().await;
    Json(students.clone())
}


pub async fn add_student( State(state): State<SharedState>,Json(mut student): Json<Student> ) -> StatusCode {

    student.id = Uuid::new_v4().to_string();

    let mut students = state.write().await;
    students.push(student);
    save_students(&students);

    StatusCode::CREATED
}


pub async fn update_student(Path(id): Path<String>,State(state): State<SharedState>, Json(updated): Json<Student>) -> StatusCode {

    let mut students = state.write().await;

    if let Some(s) = students.iter_mut().find(|s| s.id == id) {
        s.name = updated.name;
        s.email = updated.email;
        save_students(&students);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}


pub async fn delete_student(Path(id): Path<String>,State(state): State<SharedState>) -> StatusCode {

    let mut students = state.write().await;

    if students.iter().any(|s| s.id == id) {
        students.retain(|s| s.id != id);
        save_students(&students);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
