import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenStaking } from "../target/types/token_staking";
import { PublicKey, Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { createAssociatedTokenAccount, createMint, mintTo } from "@solana/spl-token";
import { assert } from "chai";

describe("token-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // TODO: use @solana/kit => solana/web3.js V2
  // TODO: when updating remember to also change import for solana/spl-token :
  //       If you are using @solana/web3.js version 2 , you should use the @solana-program/token and @solana-program/token-2022 packages instead.
  // const baseWallet = anchor.getProvider().wallet as anchor.Wallet;
  const baseWallet = anchor.getProvider().wallet;
  const poolOwner = Keypair.generate();
  const alice = Keypair.generate();
  const bob = Keypair.generate();

  const stakeTokenDecimals = 6;
  let stakeTokenMint: PublicKey;
  let stakeTokenAliceTokenAccount: PublicKey;
  let stakeTokenBobTokenAccount: PublicKey;

  const program = anchor.workspace.tokenStaking as Program<TokenStaking>;
  const connection = anchor.getProvider().connection;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Airdrop and config accounts", async () => {
    await airdrop(connection, poolOwner.publicKey);
    await airdrop(connection, alice.publicKey);
    await airdrop(connection, bob.publicKey);

    const baseWalletBalance = await connection.getBalance(baseWallet.publicKey);
    const poolOwnerBalance = await connection.getBalance(poolOwner.publicKey);
    const aliceBalance = await connection.getBalance(alice.publicKey);
    const bobBalance = await connection.getBalance(bob.publicKey);

    console.log("baseWalletBalance", baseWalletBalance);
    console.log("poolOwnerBalance", poolOwnerBalance);
    console.log("aliceBalance", aliceBalance);
    console.log("bobBalance", bobBalance);

    stakeTokenMint = await createMint(
      connection, 
      baseWallet.payer, 
      baseWallet.publicKey, // mint auth
      baseWallet.publicKey, // freeze auth
      stakeTokenDecimals
    );

    stakeTokenAliceTokenAccount = await createAssociatedTokenAccount(
      connection,
      baseWallet.payer,
      stakeTokenMint,
      alice.publicKey
    );
    stakeTokenBobTokenAccount = await createAssociatedTokenAccount(
      connection,
      baseWallet.payer,
      stakeTokenMint,
      bob.publicKey
    );

    await mintTo(
      connection,
      baseWallet.payer, // payer
      stakeTokenMint,
      stakeTokenAliceTokenAccount,
      baseWallet.publicKey, // authority
      100 * 10 ** stakeTokenDecimals,
    );
    await mintTo(
      connection,
      baseWallet.payer, // payer
      stakeTokenMint,
      stakeTokenBobTokenAccount,
      baseWallet.publicKey, // authority
      100 * 10 ** stakeTokenDecimals,
    );

    assert.equal((await connection.getTokenAccountBalance(stakeTokenAliceTokenAccount)).value.amount, "100000000");
    assert.equal((await connection.getTokenAccountBalance(stakeTokenBobTokenAccount)).value.amount,"100000000");
  });

  it("Create pool", async () => {
    let min_duration_sec = new anchor.BN(10);
    let max_duration_sec = new anchor.BN(1000);
    let max_weight_multiplier = new anchor.BN(10);

    let tx = await program.methods
      .poolCreate(min_duration_sec, max_duration_sec, max_weight_multiplier)
      .accounts({
        owner: poolOwner.publicKey,
        stakeTokenMint
      })
      .signers([poolOwner])
      .rpc()
      .catch(e => console.error());
      
    console.log("Create pool tx: ", tx);  

  });

});

export const AIRDROP_SOL_AMOUNT = 333 * LAMPORTS_PER_SOL;

export async function airdrop(connection: Connection, userPubkey: PublicKey) {
  const signature = await connection.requestAirdrop(userPubkey, AIRDROP_SOL_AMOUNT)
  const latestBlockHash = await connection.getLatestBlockhash();

  await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: signature,
  });
}
