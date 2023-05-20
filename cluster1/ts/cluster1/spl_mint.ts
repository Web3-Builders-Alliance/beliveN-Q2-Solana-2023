import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from '../wba-wallet.json';

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const payer = keypair;
const mintAuthority = keypair;


// Create a devnet connection
const connection = new Connection(
    clusterApiUrl('devnet'),
    'confirmed'
  );
    
// Specify the mint id
const mint = new PublicKey('2vCCyd8v6iRbsL5q1Xas7TLD7QRrAD1EWTBvhHKkF7GR');

// Get or create an associated token account 
async function createAccount() {
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey
      );

  console.log(`Token account created: ${tokenAccount.address.toBase58()}`);
}

// Execute the getOrCreateTokenAccount function
createAccount();


// Token Account 3EwsfFw9QqUb6yJgnPKSxPMvcxKj7THnUn7EdPDYgSKN

// Mint an associated token account 
async function mintToken() {
    const tokenAccount = new PublicKey('3EwsfFw9QqUb6yJgnPKSxPMvcxKj7THnUn7EdPDYgSKN');
    const tokenMint = await mintTo(
        connection,
        payer,
        mint,
        tokenAccount,
        mintAuthority,
        100000000000 
      )

}

mintToken();
