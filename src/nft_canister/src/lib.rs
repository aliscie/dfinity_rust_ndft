use ic_cdk_macros::*;
use ic_cdk::export::candid::{candid_method, export_service};

#[candid_method(query)]
#[ic_cdk::query]
fn say_hola(name: String) -> String {
    format!("Hello, {}!", name)
}



// #[derive(CandidType)]
// struct NFT {
//     token: Principal,
//     owner: Principal,
// }
//
// #[candid_method(query)]
// #[query]
// fn get_nft() -> NFT {
//     // Retrieve token and owner from storage
//     let token = api::call::storage_get::<u64>("token").unwrap_or(0);
//     let owner = api::call::storage_get::<Principal>("owner").unwrap_or(api::id());
//
//     NFT { token, owner }
// }
//
// #[candid_method(query)]
// #[query]
// fn get_token() -> u64 {
//     api::call::storage_get::<u64>("token").unwrap_or(0)
// }
//
// #[candid_method(query)]
// #[query]
// fn get_owner() -> Principal {
//     api::call::storage_get::<Principal>("owner").unwrap_or(api::id())
// }
//
// #[candid_method(update)]
// #[update]
// fn set_token(token: u64) {
//     api::call::storage_set("token", token);
// }
//
// #[candid_method(update)]
// #[update]
// fn set_owner(owner: Principal) {
//     api::call::storage_set("owner", owner);
// }


// #[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
// fn export_candid() -> String {
//     export_service!();
//     __export_service()
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env;
//     use std::fs::{create_dir_all, write};
//     use std::path::PathBuf;
//
//     #[test]
//     fn save_candid() {
//         let dir = PathBuf::from("/Users/ahmed/Desktop/simple_nft/src/nft_canister");
//         match create_dir_all(&dir) {
//             Ok(_) => println!("Successfully created directory"),
//             Err(e) => println!("Failed to create directory: {}", e),
//         }
//
//         let res = write(dir.join("nft_canister.did"), export_candid());
//         println!("-------- Wrote to {:?}", dir);
//         println!("-------- res {:?}", res);
//     }
// }