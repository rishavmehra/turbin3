import {Keypair} from "@solana/web3.js"

const kp = Keypair.generate()
console.log(`You've generated new Solana Wallet: ${kp.publicKey.toBase58()}`);
console.log(`[${kp.secretKey}]`)

