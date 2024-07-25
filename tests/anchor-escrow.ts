import * as anchor from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
} from "@solana/spl-token";
import { randomBytes } from "crypto";

describe("anchor-escrow", () => {
  
  // 0. Set provider, connection and program
  anchor.setProvider(anchor.AnchorProvider.env());
  const initializer = anchor.Wallet.local() as anchor.Wallet;
  const provider = anchor.getProvider();
  const connection = provider.connection;
  const program = anchor.workspace.AnchorEscrow as anchor.Program<AnchorEscrow>;


  // Determined Escrow and Vault addresses
  const seed = new anchor.BN(2);

  // 1. Boilerplate
  // Determine dummy token mints and token account addresses
  const mintZeus = Keypair.generate();
  // const mintZeus = new PublicKey("DU5rgsyMNiudhn2hf7UzgW9ahwQYBF7Ew5TZMLACjYq7");
  const escrow = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), seed.toArrayLike(Buffer, "le", 8)],
    program.programId
  )[0];
  console.log("MintZeus", mintZeus.publicKey.toBase58());
  const initializerAtaZeus = getAssociatedTokenAddressSync(mintZeus.publicKey, initializer.publicKey);
  // const escrow = new PublicKey("6DGNYG7vbkNAi3n4KkRPBtRCm1yfu2T3H42woSmZc5La");
  console.log("Escrow", escrow.toBase58());
  const zeusfrens = PublicKey.findProgramAddressSync(
    [Buffer.from("zeusfrens"), initializer.publicKey.toBuffer(), escrow.toBuffer()],
    program.programId
  )[0];
  const vault = getAssociatedTokenAddressSync(mintZeus.publicKey, escrow, true);

  // 2. Utils
  // Account Wrapper
  const accounts = {
    initializer: initializer.publicKey,
    mintZeus: mintZeus.publicKey,
    initializerAtaZeus: initializerAtaZeus,
    escrow,
    vault,
    zeusfrens,
    associatedTokenprogram: ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  };

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  it("Airdrop and create mints", async () => {
    let lamports = await getMinimumBalanceForRentExemptMint(connection);
    let tx = new Transaction();
    tx.instructions = [
      ...[mintZeus].map((m) =>
        SystemProgram.createAccount({
          fromPubkey: provider.publicKey,
          newAccountPubkey: m.publicKey,
          lamports,
          space: MINT_SIZE,
          programId: TOKEN_PROGRAM_ID,
        })
      ),
      ...[
        [mintZeus.publicKey, initializer.publicKey, initializerAtaZeus],
      ].flatMap((x) => [
        createInitializeMint2Instruction(x[0], 6, x[1], null),
        createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, x[2], x[1], x[0]),
        createMintToInstruction(x[0], x[2], x[1], 1e9),
      ]),
    ];

    await provider.sendAndConfirm(tx, [mintZeus]).then(log);
  });

  it("Initialize", async () => {
    const maxAmount = 6e6;
    const oneTimeAmount = 1e6;
    const depositAmount = 10e6;
    await program.methods
      .initialize(seed,new anchor.BN(oneTimeAmount), new anchor.BN(maxAmount), new anchor.BN(depositAmount))
      .accounts({ ...accounts })
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Claim", async () => {
    await program.methods
      .claim()
      .accounts({ ...accounts })
      .rpc()
      .then(confirm)
      .then(log);
  });
  it("Claim", async () => {
    await program.methods
      .claim()
      .accounts({ ...accounts })
      .rpc()
      .then(confirm)
      .then(log);
  });
  it("Claim", async () => {
    await program.methods
      .claim()
      .accounts({ ...accounts })
      .rpc()
      .then(confirm)
      .then(log);
  });
  it("Claim", async () => {
    await program.methods
      .claim()
      .accounts({ ...accounts })
      .rpc()
      .then(confirm)
      .then(log);
  });
  it("Claim", async () => {
    await program.methods
      .claim()
      .accounts({ ...accounts })
      .rpc()
      .then(confirm)
      .then(log);
  });
  it("Claim", async () => {
    await program.methods
      .claim()
      .accounts({ ...accounts })
      .rpc()
      .then(confirm)
      .then(log);
  });
  it("Claim", async () => {
    await program.methods
      .claim()
      .accounts({ ...accounts })
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Withdraw", async () => {
    await program.methods
      .withdraw()
      .accounts({ ...accounts })
      .rpc()
      .then(confirm)
      .then(log);

    // For Degugging Purpose

    // const latestBlockhash = await anchor
    //   .getProvider()
    //   .connection.getLatestBlockhash();

    // const ix = await program.methods
    //   .exchange()
    //   .accounts({ ...accounts })
    //   .signers([taker])
    //   .instruction()

    // const msg = new TransactionMessage({
    //   payerKey: provider.publicKey,
    //   recentBlockhash: latestBlockhash.blockhash,
    //   instructions: [ix],
    // }).compileToV0Message();

    // const tx = new VersionedTransaction(msg);
    // tx.sign([taker]);

    // console.log(Buffer.from(tx.serialize()).toString("base64"));
    // await provider.sendAndConfirm(tx).then(log);
  });

});
