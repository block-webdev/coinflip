import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Coinflip } from "../target/types/coinflip";
import {
  LAMPORTS_PER_SOL,
  PublicKey,
  Keypair,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
  clusterApiUrl,
  Connection
} from "@solana/web3.js";

const VAULT_SEED = "VAULT_SEED";
const USER_STATE_SEED = "USER_STATE_SEED";


describe("coinflip", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Coinflip as Program<Coinflip>;
  let user = Keypair.generate();
  let admin = Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    await airdropSol(provider, user.publicKey, 10000000000); // 10 sol
    await airdropSol(provider, admin.publicKey, 10000000000);

    const [vaultKey] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(VAULT_SEED)],
      program.programId
    );
    console.log(vaultKey.toBase58());
    const [userStateKey] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(USER_STATE_SEED)],
      program.programId
    );


    let tx = new Transaction().add(
      await program.methods
        .initialize()
        .accounts({
          authority: admin.publicKey,
          userState: userStateKey,
          vault: vaultKey,
          systemProgram: SystemProgram.programId
        })
        .instruction()
    );


    await airdropSol(provider, vaultKey, 10000000000000);

    let txHash = await sendAndConfirmTransaction(provider.connection, tx, [admin]);
    console.log("Your transaction signature", txHash);

    let min = 100, max = 200;
    min = Math.ceil(min);
    max = Math.floor(max);
    let rand = Math.floor(Math.random() * (max - min) + min);

    console.log('22222222222', rand);

    tx = new Transaction().add(
      await program.methods
        .coinflip(new anchor.BN(10000000000), rand)
        .accounts({
          vault: vaultKey,
          userState: userStateKey,
          user: user.publicKey,
          systemProgram: SystemProgram.programId
        })
        .instruction()
    );

    txHash = await sendAndConfirmTransaction(provider.connection, tx, [admin]);
    console.log("Your transaction signature", txHash);

    let userData = await program.account.userState.fetch(userStateKey);
    console.log('user data : ', userData);
    console.log('rewards : ', userData.lastRewards.toNumber());
  });
});



const airdropSol = async (provider: anchor.Provider, target: anchor.web3.PublicKey, lamps: number) => {
  const sig: string = await provider.connection.requestAirdrop(target, lamps);
  await provider.connection.confirmTransaction(sig);
  return sig;
}
