mod exception;

use std::{
    io::{Error, ErrorKind},
    time,
};

use exception::RpcError;

fn mock_rpc_error(dummy: u16) -> Result<(), RpcError> {
    if dummy < 10 {
        return Err(RpcError::Timeout(time::Duration::from_millis(1000)));
    }

    if dummy < 20 {
        return Err(RpcError::AclError("ad.meta.info".to_owned()));
    }

    if dummy < 30 {
        return Err(RpcError::LoadbalanceError);
    }

    if dummy < 40 {
        return Err(RpcError::UnknownError("unknown error".to_string()));
    }

    if dummy < 50 {
        return Err(RpcError::SysError(Error::new(
            ErrorKind::Unsupported,
            "invalid request",
        )));
    }

    if dummy < 60 {
        return Err(RpcError::BizError(1024));
    }

    Ok(())
}

fn wrap(dummy: u16) {
    let res = mock_rpc_error(dummy);
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("error:{:?}", e.to_string())
        }
    }
}

fn ret_anyhow(dummy: u16) -> anyhow::Result<()> {
    let res = mock_rpc_error(dummy);

    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn main() {
    wrap(5);
    wrap(15);
    wrap(25);
    wrap(35);
    wrap(45);
    wrap(55);
    wrap(100);

    let res = ret_anyhow(10);

    if let Err(e) = res {
        println!("{:?}", e);
    }
}
