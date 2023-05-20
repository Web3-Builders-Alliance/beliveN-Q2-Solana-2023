import { Metaplex, keypairIdentity, bundlrStorage } from "@metaplex-foundation/js";
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import wallet from '../wba-wallet.json';



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
        const { nft } = await metaplex.nfts().create(
            {
                uri: "https://bafybeigajorfa5fr354vkryn3pd4rinq6gxdilb55xkiy7eu2obzazfjca.ipfs.nftstorage.link/rug.json",
                name: "beliven rug",
                symbol : "BR",
                creators: [{address: keypair.publicKey,
                    share: 100,}],
                sellerFeeBasisPoints: 500,
                isMutable: true
            }
        )
        console.log(nft.address)

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }


})();

