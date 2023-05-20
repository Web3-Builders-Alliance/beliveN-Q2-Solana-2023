import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { createMint } from '@solana/spl-token';
import wallet from '../wba-wallet.json';


// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const payer = keypair;

// Create a devnet connection
const connection = new Connection(
  clusterApiUrl('devnet'),
  'confirmed'
);


// Specify the mint authority account
const mintAuthority = keypair;
const freezeAuthority = keypair;


// Create a new token mint
async function create() {
    // Define the token mint parameters 
    const tokenMint = await createMint(
        connection,
        payer,
        mintAuthority.publicKey,
        freezeAuthority.publicKey,
        6 
    );
  
    // Console log the mint id
    console.log(`Mint ID: ${tokenMint.toBase58()}`);
  }
  

create();