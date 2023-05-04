use actix_web::{HttpResponse, Responder, web};
use log::error;
use rbatis::rbdc::Error;
use serde::Deserialize;
use std::default::Default;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use actix_web::web::Json;
use rbatis::sql::IPage;

#[derive(Deserialize, Clone, Copy, Default)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    #[default]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}

#[derive(Deserialize)]
pub struct Page {
    #[serde(default = "page_default")]
    pub page: u64,
    #[serde(default = "size_default")]
    pub size: u64,
    pub ordering: Option<String>,
    #[serde(default)]
    pub sort_order: SortOrder,
}

fn page_default() -> u64 { 1 }

fn size_default() -> u64 { 10 }

pub fn return_paginated_data<T>(entities: Result<rbatis::sql::Page<T>, Error>) -> impl Responder
    where
        T: serde::Serialize,
{
    match entities {
        Ok(results) => {
            HttpResponse::Ok().json(&results)
        }
        Err(err) => {
            error!("An error occured : {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn return_single_data<T>(entity: Result<Option<T>, Error>) -> impl Responder
    where
        T: serde::Serialize,
{
    match entity {
        Ok(result) => {
            match result {
                None => HttpResponse::NotFound().finish(),
                Some(e) => HttpResponse::Ok().json(e)
            }
        }
        Err(err) => {
            error!("An error occured : {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn build_result_page<T>(page_no: u64, page_size: u64, total: u64, records: Vec<T>) -> rbatis::sql::Page<T> {
    rbatis::sql::Page::<T>::new_total(page_no, page_size, total)
        .set_records(records)
}

pub struct DatabaseError {
    error: Error,
}

impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl actix_web::error::ResponseError for DatabaseError {}

impl From<Error> for DatabaseError {
    fn from(error: Error) -> Self {
        DatabaseError { error }
    }
}