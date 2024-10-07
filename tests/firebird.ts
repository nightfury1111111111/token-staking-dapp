import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { 
  createMint, 
  mintTo,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { assert } from "chai";
import { Firebird } from "../target/types/firebird";

describe("firebird", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Firebird as Program<Firebird>;

  const connection = anchor.getProvider().connection;

  const provider = anchor.getProvider();

  const signer = anchor.web3.Keypair.generate();

  before(async () => {
    const airdropTx = await connection.requestAirdrop(
      signer.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropTx);
  });

  it("Initializes the global authority", async () => {
    await program.methods
      .initialize()
      .accounts({
        admin: signer.publicKey,
      })
      .signers([signer])
      .rpc();

    const [globalAuthority, _] = PublicKey.findProgramAddressSync(
      [Buffer.from("Global Pool")],
      program.programId
    );
    const globalAuthorityAccount = await program.account.globalPool.fetch(
      globalAuthority
    );

    assert.equal(
      signer.publicKey.toString(),
      globalAuthorityAccount.admin.toString()
    );
  });

  it("Initializes user token pool", async () => {
    const mint = await createMint(
      provider.connection,
      signer,
      signer.publicKey,
      null,
      9
    );

    await program.methods
      .userTokenPoolInitialize()
      .accounts({
        signer: signer.publicKey,
        tokenMint: mint,
      })
      .signers([signer])
      .rpc();
  });

  it("Deposit and then withdraw memecoin", async () => {
    const user = anchor.web3.Keypair.generate();
    const airdropTx = await connection.requestAirdrop(
      user.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropTx);

    const mint = await createMint(
      provider.connection,
      user,
      user.publicKey,
      null,
      9
    );

    const userTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mint,
      user.publicKey
    );

    await mintTo(
      provider.connection,
      user,
      mint,
      userTokenAccount.address,
      user.publicKey,
      1000000000000
    )

    const [tokenPoolAddress, _] = PublicKey.findProgramAddressSync(
      [Buffer.from("Global Pool")],
      program.programId
    )

    const poolTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer,
      mint,
      tokenPoolAddress,
      true
    )

    await program.methods
      .userTokenPoolInitialize()
      .accounts({
        signer: user.publicKey,
        tokenMint: mint,
      })
      .signers([user])
      .rpc();

    const depositAmount = new BN(1000000000000);

    await program.methods.deposit(depositAmount).accounts({
      userTokenAccount: userTokenAccount.address,
      tokenMint: mint,
      tokenPool: poolTokenAccount.address,
      signer: user.publicKey
    }).signers([user]).rpc()

    const withdrawAmount = new BN(100000000000);

    await program.methods.withdraw(withdrawAmount).accounts({
      userTokenAccount: userTokenAccount.address,
      tokenMint: mint,
      tokenPool: poolTokenAccount.address,
      signer: user.publicKey
    }).signers([user]).rpc();

    const poolBalance = await connection.getTokenAccountBalance(poolTokenAccount.address);

    console.log("pool token balance", poolBalance)
  })
});
