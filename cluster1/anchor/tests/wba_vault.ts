import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WbaVault, IDL } from "../target/types/wba_vault";




describe("wba_vault", () => {
  // Configure the client to use the local cluster.
    // Configure the client to use the local cluster.
    const keypair = anchor.web3.Keypair.generate();
  
  const connection = new anchor.web3.Connection("http://localhost:8899");
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(keypair), { commitment: "finalized" } );
  const programId = new anchor.web3.PublicKey("4TLhT3e1bn8UcSg6qQScKhWi1LwEy52yy7cFn6J1rt7S");
  const program = new anchor.Program<WbaVault>(IDL, programId, provider);
  

  

  const vaultState =anchor.web3.Keypair.generate();

// Create PDA VAULT AUTH

const vault_auth_seeds = [Buffer.from("auth"), vaultState.publicKey.toBuffer()];
const vault_auth = anchor.web3.PublicKey.findProgramAddressSync(vault_auth_seeds, program.programId)[0];

// Create Vault system Program
const vault_seeds = [Buffer.from("vault"), vault_auth.toBuffer()];
const vault = anchor.web3.PublicKey.findProgramAddressSync(vault_seeds, program.programId)[0];



it("Starts an airdrop and confirms it", async () => {
  // 1. Airdrop 100 SOL to payer
  const signature = await provider.connection.requestAirdrop(keypair.publicKey, 0.2 * 1_000_000_000);
  const latestBlockhash = await connection.getLatestBlockhash();
  await provider.connection.confirmTransaction(
  {
      signature,
      ...latestBlockhash,
  },
"finalized"
  );  
})


  it("Is initialized!", async () => {
    // Add your test here.
    try {
    const txhash = await program.methods
    .initialize()
    .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vault_auth,
        vault:vault,
        systemProgram: anchor.web3.SystemProgram.programId,
    
    })
    .signers([
        keypair,
        vaultState

    ]).rpc();   
    console.log(`Success! Check out your TX here: 
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }
    catch(e){console.error(e)}
  });
});
