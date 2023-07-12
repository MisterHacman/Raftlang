#[macro_export]
macro_rules! match_maybe {
    ($ex:expr, $err:expr) => {
        match $ex {
            Some(some) => some,
            None => return Err($err)
        }
    }
}

#[macro_export]
macro_rules! match_either {
    ($ex:expr) => {
        match $ex {
            Ok(ok) => ok,
            Err(err) => return Err(err)
        }
    };
    ($ex:expr, $buf:expr, $t:ty) => {
        match $ex {
            Ok(ok) => ok,
            Err(err) => return Err((err as $t).as_str($buf))
        }
    };
    ($ex:expr, $err:expr) => {
        match $ex {
            Ok(ok) => ok,
            Err(_) => return Err($err)
        }
    };
}

pub fn both<F, G>(a: F, b: G) -> bool where F: Fn() -> bool, G: Fn() -> bool {
    let aval = a();
    if aval { b() } else { aval }
}
