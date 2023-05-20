import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, clusterApiUrl } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection(
    clusterApiUrl('devnet'),
    'confirmed'
  );
    

// Mint address
const mint = new PublicKey("2vCCyd8v6iRbsL5q1Xas7TLD7QRrAD1EWTBvhHKkF7GR");

// Recipient address
const to = new PublicKey("EW2vU89PJYDTAaqhbf2RSL5JbGREt4EoUq68wX8R8Dfa");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it

        const from_tokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );

        // Get the token account of the toWallet address, and if it does not exist, create it

        const to_tokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );

        // Transfer the new token to the "toTokenAccount" we just created

        const txhash = transfer(
            connection,
            keypair,
            from_tokenAccount.address,
            to_tokenAccount.address,
            keypair.publicKey,
            1000

        );
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();