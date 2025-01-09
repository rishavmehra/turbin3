mod programs;

#[cfg(test)]
mod tests {
    use::std::io::{self, BufRead};
    use std::str::FromStr;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{self, bs58, message::Message, signature::{read_keypair_file, Keypair}, signer::Signer, system_program, sysvar::recent_blockhashes, transaction::Transaction};
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use crate::programs::Turbin3_prereq::{
        Turbin3PrereqProgram, CompleteArgs, UpdateArgs
    };


    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've genrated new solana wallet: {}", kp.pubkey().to_string()); println!("");
        println!("to save you wallet, copy and paste following in to the json file:");
        println!("{:?}", kp.to_bytes());
    }
    
    #[test]
    fn base_to_wallet(){
        println!("Input you private key as base58");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();println!("{:?}", wallet)
    }

    #[test]
    fn wallet_to_base58(){
        println!("Input you private key as a wallet file byte array:"); let stdin = io::stdin();
        let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',') .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        print!("Your private key is: ");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58)
    }


    #[test]
    fn airdrop(){
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("dev-wallet.json").expect("couldn't find wallet file");

        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64){
            Ok(s)=>{
                println!("Success checkout your transaction here: ");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Something went wrong: {}", e.to_string())
    }
}
    
    #[test]
    fn transfer_sol(){
        const RPC_URL: &str = "https:api.devnet.solana.com";
        let keypair = read_keypair_file("dev-wallet.json").expect("couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("85Wnyyd6RNm7mow351m6hgnfQtTgUJWUph1FkZMh5Spy").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                            &keypair.pubkey(), 
                            &to_pubkey, 
                            1_000_000
                        )],
                        Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
            );

        let sign = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send Transaction");

        println!("Success! Check out your Tx here: https://explorer.solana.com/tx/{}?cluster=devnet", sign);
        


    }

    #[test]
    fn full_transfer_sol(){
        const RPC_URL: &str = "https:api.devnet.solana.com";
        let keypair = read_keypair_file("dev-wallet.json").expect("couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("85Wnyyd6RNm7mow351m6hgnfQtTgUJWUph1FkZMh5Spy").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        // let transaction = Transaction::new_signed_with_payer(
        //     &[transfer(
        //                     &keypair.pubkey(), 
        //                     &to_pubkey, 
        //                     1_000_000
        //                 )],
        //                 Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
        //     );
        
        let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get the balance");
    
    
    
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)], 
            Some(&keypair.pubkey()), 
            &recent_blockhash
        );

        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                            &keypair.pubkey(), 
                            &to_pubkey, 
                            balance-fee,
                        )],
                        Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
            );
                
        let sign = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send Transaction");

        println!("Success! Check out your Tx here: https://explorer.solana.com/tx/{}?cluster=devnet", sign);
        
    }

    #[test]
    fn enroll(){
        const RPC_URL: &str = "https:api.devnet.solana.com";
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("turbin3-wallet.json").expect("Couldn't find wallet file");

        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

        let args = CompleteArgs{
            github: b"rishavmehra".to_vec()
        };


        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        let sign = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send Transaction");

        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", sign)

    }


}


