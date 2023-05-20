import { Metaplex, keypairIdentity, bundlrStorage, toMetaplexFile } from "@metaplex-foundation/js";
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import wallet from '../wba-wallet.json';
import { readFile } from "fs/promises"



// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));


// Create a devnet connection
const connection = new Connection(
  clusterApiUrl('devnet'),
  'confirmed'
);

const metaplex = Metaplex.make(connection)
    .use(keypairIdentity(keypair))
    .use(bundlrStorage({
        address: 'https://devnet.bundlr.network',
        providerUrl: "https://api.devnet.solana.com",
        timeout: 60000,
    }));



    (async () => {
        try {
            const img = await readFile("./images/rug.json");
            const metaplexImg = toMetaplexFile(img,"rug.json");
            const imageURI = await metaplex.storage().upload(metaplexImg)
  
            console.log(imageURI)
    
        } catch(e) {    
            console.error(`Oops, something went wrong: ${e}`)
        }
    
    
    })();
    
    