#[macro_export]
macro_rules! match_res {
    ($ex:expr, $o:ty, $e:ty) => {
        match $ex {
            Ok(ok) => ok as $o,
            Err(err) => { return Err(err) }
        }
    };
}

pub fn both<F, G>(a: F, b: G) -> bool where F: Fn() -> bool, G: Fn() -> bool {
    let aval = a();
    if aval { b() } else { aval }
}
