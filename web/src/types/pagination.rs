use crate::error;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pagination {
    pub limit: Option<u32>,
    pub offset: u32,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, error::Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(error::Error::ParseError)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(error::Error::ParseError)?,
        });
    }

    Err(error::Error::MissingParameters)
}
