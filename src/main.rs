use axum::{
    extract::{self, Query},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Serialize, Deserialize, Debug)]
struct JsonData {
    #[serde(rename = "Number", default)]
    number: usize,
    #[serde(rename = "String", default)]
    string: String,
    #[serde(rename = "Bool", default)]
    boolean: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddData {
    right: i64,
    left: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/hello/:username", get(hello_handler))
        .route("/json", get(json_handler))
        .route("/ping", get(|| async { "pong" }))
        .route("/fizzbuzz", get(fizzbuzz_handler))
        .route("/add", post(add_handler))
        .route("/students/:class/:studentNumber", get(students_handler));
    axum::Server::bind(&"0.0.0.0:57577".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn hello_handler(extract::Path(username): extract::Path<String>) -> String {
    format!("hello, {}!", username)
}
async fn json_handler() -> (StatusCode, Json<JsonData>) {
    let res = JsonData {
        number: 10,
        string: "hoge".to_string(),
        boolean: false,
    };
    (StatusCode::OK, Json(res))
}
async fn fizzbuzz_handler(Query(params): Query<HashMap<String, String>>) -> (StatusCode, String) {
    let cnt = params.get("count").unwrap_or(&"30".to_string()).to_owned();
    let cnt = match cnt.parse::<i64>() {
        Ok(x) => x,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Bad Request".to_string());
        }
    };
    let s = {
        let mut s = String::new();
        for i in 1..=cnt {
            if i % 3 == 0 {
                s += "fizz";
            }
            if i % 5 == 0 {
                s += "buzz";
            }
            if (i % 5) * (i % 3) != 0 {
                s += &i.to_string();
            }
            s += "\n";
        }
        s
    };
    (StatusCode::OK, s)
}

async fn add_handler(Json(json): Json<Value>) -> (StatusCode, Json<Value>) {
    let add_data = match serde_json::from_value::<AddData>(json) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Bad Request"})),
            )
        }
    };
    (
        StatusCode::OK,
        Json(json!({"answer": add_data.left+add_data.right })),
    )
}
struct Class {
    class_number: usize,
    students: Vec<Student>,
}
struct Student {
    student_number: usize,
    name: String,
}

async fn students_handler(
    extract::Path((class, studentnumber)): extract::Path<(usize, usize)>,
) -> (StatusCode, Json<Value>) {
    let classes = [
        Class {
            class_number: 1,
            students: vec![
                Student {
                    student_number: 1,
                    name: "hijiki51".to_string(),
                },
                Student {
                    student_number: 2,
                    name: "logica".to_string(),
                },
                Student {
                    student_number: 3,
                    name: "Ras".to_string(),
                },
            ],
        },
        Class {
            class_number: 2,
            students: vec![
                Student {
                    student_number: 1,
                    name: "asari".to_string(),
                },
                Student {
                    student_number: 2,
                    name: "irori".to_string(),
                },
                Student {
                    student_number: 3,
                    name: "itt".to_string(),
                },
                Student {
                    student_number: 4,
                    name: "mehm8128".to_string(),
                },
            ],
        },
        Class {
            class_number: 3,
            students: vec![
                Student {
                    student_number: 1,
                    name: "reyu".to_string(),
                },
                Student {
                    student_number: 2,
                    name: "yukikurage".to_string(),
                },
                Student {
                    student_number: 3,
                    name: "anko".to_string(),
                },
            ],
        },
        Class {
            class_number: 4,
            students: vec![
                Student {
                    student_number: 1,
                    name: "Uzaki".to_string(),
                },
                Student {
                    student_number: 2,
                    name: "yashu".to_string(),
                },
            ],
        },
    ];
    let v = classes.iter().find(|&x| x.class_number == class);
    let cls = match v {
        Some(x) => x,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Student Not Found"})),
            )
        }
    };
    let v = cls
        .students
        .iter()
        .find(|&x| x.student_number == studentnumber);
    let st = match v {
        Some(x) => x,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"Student Not Found"})),
            )
        }
    };
    (
        StatusCode::OK,
        Json(json!({"student_number": st.student_number,"name": st.name})),
    )
}
