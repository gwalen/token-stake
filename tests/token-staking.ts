import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenStaking } from "../target/types/token_staking";
import { PublicKey, Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { createAssociatedTokenAccount, createMint, getAssociatedTokenAddressSync, mintTo } from "@solana/spl-token";
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

  const STAKE_TOKEN_DECIMALS = 6;
  let stakeTokenMint: PublicKey;
  let stakeTokenAliceTokenAccount: PublicKey;
  let stakeTokenBobTokenAccount: PublicKey;
  let poolConfigPda: PublicKey;

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
      STAKE_TOKEN_DECIMALS
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
      100 * 10 ** STAKE_TOKEN_DECIMALS,
    );
    await mintTo(
      connection,
      baseWallet.payer, // payer
      stakeTokenMint,
      stakeTokenBobTokenAccount,
      baseWallet.publicKey, // authority
      100 * 10 ** STAKE_TOKEN_DECIMALS,
    );

    assert.equal((await connection.getTokenAccountBalance(stakeTokenAliceTokenAccount)).value.amount, "100000000");
    assert.equal((await connection.getTokenAccountBalance(stakeTokenBobTokenAccount)).value.amount,"100000000");
  });

  it("Create pool", async () => {
    let min_duration_sec = new anchor.BN(10);
    let max_duration_sec = new anchor.BN(1010); // 1000 + 10 so that we have total max_duration period == 1000
    let max_weight_multiplier = new anchor.BN(10);

    let tx = await program.methods
      .poolCreate(min_duration_sec, max_duration_sec, max_weight_multiplier)
      .accounts({
        owner: poolOwner.publicKey,
        stakeTokenMint
      })
      .signers([poolOwner])
      .rpc()
      .catch(e => console.error(e));
      
    console.log("Create pool tx: ", tx);  

    let user_stake = program.account.userStake.fetch();

  });

  it("Stake user tokens", async () => {
    let stakeAmount = new anchor.BN(10 * STAKE_TOKEN_DECIMALS);
    let userLockupPeriodSec = new anchor.BN(510); // + 10 to adjust for the pool_config.min_duration
    poolConfigPda = derivePoolConfigPda(program.programId, poolOwner.publicKey, stakeTokenMint);

    let aliceStakeTokenAta = getAssociatedTokenAddressSync(
      stakeTokenMint,
      alice.publicKey
    );

    let poolStakeTokenAta = getAssociatedTokenAddressSync(
      stakeTokenMint,
      poolConfigPda,
      true // allow off curve (pda)
    );

    let userTokenAmountBefore = Number((await connection.getTokenAccountBalance(aliceStakeTokenAta)).value.amount)
    let poolTokenAmountBefore = Number((await connection.getTokenAccountBalance(poolStakeTokenAta)).value.amount)

    let tx = await program.methods
      .stakeTokens(stakeAmount, userLockupPeriodSec)
      .accounts({
        user: alice.publicKey,
        poolOwner: poolOwner.publicKey,
        stakeTokenMint
      })
      .signers([alice])
      // .rpc({ skipPreflight: true });
      .rpc()
      .catch(e => console.error(e));

    console.log("Stake alice tokens tx sig: ", tx);

    let userStakePda = deriveUserStakePda(program.programId, alice.publicKey, stakeTokenMint);
    let userStake = await program.account.userStake.fetch(userStakePda);

    let userTokenAmountAfter = Number((await connection.getTokenAccountBalance(aliceStakeTokenAta)).value.amount);
    let poolTokenAmountAfter = Number((await connection.getTokenAccountBalance(poolStakeTokenAta)).value.amount);

    /**
     * Given:
      pool_config.min_duration_sec = 10
      pool_config.max_duration_sec = 1010
      pool_config.max_weight_multiplier = 10 (Assuming this means 10x, not 10 BIPS)
      user_lockup_period_sec = 510
      Steps:
      1. Check if user_lockup_period_sec is within the pool's range:
      10 <= 510 <= 1010. Yes, it is. So we use user_lockup_period_sec directly (no clamping needed in this case).
      2. Calculate the total duration range eligible for weight increase:
      total_duration_range = max_duration - min_duration = 1010 - 10 = 1000 seconds.
      3. Calculate how far the user's lockup period is into that range:
      adjusted_lockup_period = user_lockup_period - min_duration = 510 - 10 = 500 seconds.
      4. Calculate the total possible weight increase above the base 1x:
      weight_increase_range = max_weight_multiplier - 1 = 10 - 1 = 9.
      5. Calculate the user's achieved portion of the maximum increase:
      increase_ratio = adjusted_lockup_period / total_duration_range = 500 / 1000 = 0.5. (The user locked up for exactly half of the duration range).
      6. Calculate the actual weight increase for the user:
      actual_increase = weight_increase_range * increase_ratio = 9 * 0.5 = 4.5.
      7. Add the base weight (1x) to the actual increase:
      weight_multiplier = 1 + actual_increase = 1 + 4.5 = 5.5.
      8. Result:
      The calculated weight_multiplier for a user locking up 510 seconds is 5.5x.
     */
    assert.equal(userStake.weightMultiplier.toNumber() / BIPS, 5.5);
    assert.equal(userStake.endTime.toNumber(), userStake.startTime.toNumber() + userLockupPeriodSec.toNumber());
    
    let userTokenDiff = Math.abs(userTokenAmountAfter - userTokenAmountBefore);
    let poolTokenDiff = Math.abs(poolTokenAmountAfter - poolTokenAmountBefore);
    assert.equal(userTokenDiff, stakeAmount.toNumber());
    assert.equal(poolTokenDiff, stakeAmount.toNumber());
  });

});

export const AIRDROP_SOL_AMOUNT = 333 * LAMPORTS_PER_SOL;

export const BIPS = 10_000;

export async function airdrop(connection: Connection, userPubkey: PublicKey) {
  const signature = await connection.requestAirdrop(userPubkey, AIRDROP_SOL_AMOUNT)
  const latestBlockHash = await connection.getLatestBlockhash();

  await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: signature,
  });
}

export function deriveUserStakePda(
  programId: PublicKey,
  user: PublicKey,
  stake_token_mint: PublicKey
): PublicKey {
  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user_stake"), user.toBuffer(), stake_token_mint.toBuffer()],
    programId
  );
  return pda;
}

export function derivePoolConfigPda(
  programId: PublicKey,
  pool_owner: PublicKey,
  stake_token_mint: PublicKey
): PublicKey {
  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("pool_config"), pool_owner.toBuffer(), stake_token_mint.toBuffer()],
    programId
  );
  return pda;
}
