use ic_cdk::export::Principal;
use ic_cdk::export::candid::{candid_method, export_service};
use ic_cdk_macros::*;

#[candid_method(query)]
#[ic_cdk::query]
async fn greet(name: String) -> String {
    // let canister_a_id_str = "br5f7-7uaaa-aaaaa-qaaca-cai";
    // let canister_a_id = Principal::from_text(canister_a_id_str).unwrap();
    //
    // let my_const: String = ic_cdk::api::call::call(
    //     canister_a_id,
    //     "get_my_const",
    //     (),
    // ).await.unwrap();
    let my_const = "xx";

    format!("Hello, {}! {}", my_const, name)
}

// #[candid_method(update)]
// #[update]
// fn mint_nft(token: u64, owner: Principal) {
//     // Store the token and owner in stable memory
//     let nft = NFT { token, owner };
//     api::call::stable_store("nft", nft);
// }
//
// #[candid_method(query)]
// #[query]
// fn get_nft() -> Option<NFT> {
//     // Retrieve the NFT from stable memory
//     api::call::stable_load::<NFT>("nft")
// }


#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{create_dir_all, write};
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn save_candid() {
        let dir = PathBuf::from("/Users/ahmed/Desktop/simple_nft/src/simple_nft_backend");
        match create_dir_all(&dir) {
            Ok(_) => println!("Successfully created directory"),
            Err(e) => println!("Failed to create directory: {}", e),
        }

        let res = write(dir.join("simple_nft_backend.did"), export_candid());
        println!("-------- Wrote to {:?}", dir);
        println!("-------- res {:?}", res);
    }
}