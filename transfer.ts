// import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";

// import wallet from "./dev-wallet.json"

// const from = Keypair.fromSecretKey(new Uint8Array(wallet))
// const to = new PublicKey("85Wnyyd6RNm7mow351m6hgnfQtTgUJWUph1FkZMh5Spy");

// const connection = new Connection("https://api.devnet.solana.com");

// (async () => {
//     try {

//         const balance = await connection.getBalance(from.publicKey)

//         const transaction = new Transaction().add(
//             SystemProgram.transfer({
//                 fromPubkey: from.publicKey,
//                 toPubkey: to,
//                 lamports: balance
//             })
//         )
//         transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
//         transaction.feePayer = from.publicKey;

//         const fee = (await connection.getFeeForMessage(transaction.compileMessage(),'confirmed')).value || 0;
//         transaction.instructions.pop;

//         transaction.add(
//             SystemProgram.transfer({
//                 fromPubkey: from.publicKey,
//                 toPubkey: to,
//                 lamports: balance - fee,
//             })
//         )

//         const signature = await sendAndConfirmTransaction(
//             connection,
//             transaction,
//             [from]
//         )

//         console.log(`Checkout you transaction Here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);

//     } catch (e) {
//         console.error(`Something went wrong: ${e}`)
//     }

// })();




import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";

import wallet from "./dev-wallet.json"

const from = Keypair.fromSecretKey(new Uint8Array(wallet))
const to = new PublicKey("85Wnyyd6RNm7mow351m6hgnfQtTgUJWUph1FkZMh5Spy");

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        const balance = await connection.getBalance(from.publicKey);

        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance
            })
        );

        transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
        transaction.feePayer = from.publicKey;

        // Get the fee estimate
        const fee = (await connection.getFeeForMessage(transaction.compileMessage(), 'confirmed')).value || 0;

        // Create a new transaction with the adjusted lamport amount
        const adjustedTransaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance - fee, // Subtract the fee from the balance
            })
        );

        adjustedTransaction.recentBlockhash = transaction.recentBlockhash;
        adjustedTransaction.feePayer = from.publicKey;

        const signature = await sendAndConfirmTransaction(
            connection,
            adjustedTransaction,
            [from]
        );

        console.log(`Checkout your transaction here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);

    } catch (e) {
        console.error(`Something went wrong: ${e}`);
    }
})();
