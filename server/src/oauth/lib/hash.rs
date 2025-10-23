// use once_cell::sync::Lazy;
use std::hash::{DefaultHasher, Hash, Hasher};
// use std::sync::{Mutex, MutexGuard, PoisonError};

// static HASHER: Lazy<Mutex<DefaultHasher>> = Lazy::new(|| Mutex::new(DefaultHasher::new()));

pub enum HashErr {}

// /// TODO: looks like i can't get a &mut to a MutexGuard, so this doesn't work
// fn get_hasher(
// ) -> Result<MutexGuard<'static, DefaultHasher>, PoisonError<MutexGuard<'static, DefaultHasher>>> {
//     HASHER.lock()
// }

/// Hash the given string
pub fn hash(s: String) -> Result<String, HashErr> {
    // let mut h = match get_hasher() {
    //     Ok(h) => h,
    //     Err(e) => {
    //         eprintln!("Error while getting hasher: {e}");
    //         return Err(HashErr::HashErr);
    //     }
    // };
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    Ok(h.finish().to_string())
}
