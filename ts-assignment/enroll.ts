

import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import wallet from "./turbin3-wallet.json"
import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor"
import { IDL, TurbinPreReq } from "./programs/Turbin3_prereq"
const kp = Keypair.fromSecretKey(new Uint8Array(wallet))


const connection = new Connection("https://api.devnet.solana.com")
const github = Buffer.from("rishavmehra", "utf-8")

const provider = new AnchorProvider(connection, new Wallet(kp), { commitment: "confirmed" });


const program: Program<TurbinPreReq> = new Program(IDL, provider)

const enrollment_seeds = [Buffer.from("prereq"),
kp.publicKey.toBuffer()];

const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(enrollment_seeds, program.programId);
(async () => {
    try {
        const txHash = await program.methods
            .complete(github)
            .accounts({
                signer: kp.publicKey,
            })
            .signers([
                kp
            ]).rpc();
        console.log(`Check the transaction here: https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
    } catch(e) {
        console.error(`Something is wrong: ${e}`)
    }
})();
