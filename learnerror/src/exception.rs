use std::time;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("rpc timeout with {0:?}")]
    Timeout(time::Duration),

    #[error("request from {0} is not allowed")]
    AclError(String),

    #[error("load balance fail")]
    LoadbalanceError,

    #[error("system error: {0:?}")]
    SysError(#[from] std::io::Error),

    #[error("business error:{0}")]
    BizError(u32),

    #[error("unknown error:{0}")]
    UnknownError(String),
}
