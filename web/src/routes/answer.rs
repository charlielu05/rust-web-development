use crate::store::Store;
use crate::types::answer::{Answer, AnswerId};
use crate::types::question::QuestionId;
use std::collections::HashMap;
use warp::http::StatusCode;

pub async fn add_answer(
    store: Store,
    params: HashMap<String, i32>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        id: AnswerId(1),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("questionId").unwrap().to_owned()),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
