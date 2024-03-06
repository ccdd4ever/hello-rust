pub mod abi;

pub use abi::*;
use thiserror::Error;
use crate::pb::command_request::RequestCmd;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_cmd {
        Some(RequestCmd::Hget(param)) => param.execute(store),
        None => KvError::InvalidCmd("".into()).into(),
        _ => KvError::InvalidCmd("Not implemented".into()).into(),
    }
}

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound.into(),
            Err(e) => e.into()
        }
    }
}

impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_cmd: Some(RequestCmd::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }
}


impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            value: Some(value::Value::String(value)),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            value: Some(value::Value::String(value.into()))
        }
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum KvError {
    #[error("NotFound")]
    NotFound,
    #[error("Invalid command:`{0}`")]
    InvalidCmd(String),
    #[error("Internal error")]
    Internal,
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: 200,
            message: "".to_string(),
            values: vec![value],
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: 500,
            message: e.to_string(),
            values: vec![],
            kvparis: vec![],
        };
        match e {
            KvError::InvalidCmd(_) => result.status = 401,
            KvError::Internal => result.status = 500,
            _ => {}
        }

        result
    }
}

pub trait Storage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn set(&self, table: &str, key: &str, value: Value) -> Result<Option<Value>, KvError>;
    fn contains(&self, table: &str, key: &str) -> Result<Option<bool>, KvError>;
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    //dyn+Trait表示一个Trait的实现（trait object)
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item=Kvpair>>, KvError>;
}