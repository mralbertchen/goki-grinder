use std::convert::TryFrom;

use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
};

fn main() {
    let to_find = "ab".to_ascii_lowercase();
    let char_length = to_find.len();

    let mut i = 0;
    let mut found = false;

    while !found {
        let keypair = Keypair::new();
        let program_id = Pubkey::try_from("GokivDYuQXPZCWRkwMhdH2h91KpDQXBEmpgBgs55bnpH").unwrap();

        let (pda, _bump) = Pubkey::find_program_address(
            &[
                b"GokiSmartWallet".as_ref(),
                keypair.pubkey().to_bytes().as_ref(),
            ],
            &program_id,
        );

        let pda_string = pda.to_string();
        println!("attempt {}, the pda is: {}", i + 1, pda_string);
        let to_match = &pda_string[..char_length].to_ascii_lowercase();

        if to_match.eq(&to_find) {
            found = true;
        }

        i += 1;
    }
}
