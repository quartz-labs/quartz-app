import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor";
import { FundsProgram } from "../target/types/funds_program";
import { Keypair, SystemProgram, LAMPORTS_PER_SOL, Transaction, PublicKey } from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID, mintTo, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import dotenv from 'dotenv';
const fs = require("fs");
import path from "path";
import { assert, expect } from "chai";
import { setupTests } from "./setup_tests";
dotenv.config();


describe("init_user tests", () => {
  let testSetup: Awaited<ReturnType<typeof setupTests>>;

  before(async () => {
    testSetup = await setupTests();
  });

  it("init_user incorrect signer", async () => {
    const {program, otherKeypairVaultUsdcPda, otherOwnerKeypair, otherKeypairVaultPda, testUsdcMint, quartzManagerKeypair} = testSetup;
    const desiredErrorMessage = "Missing signature";

    try {
      const tx = await program.methods
        .initUser()
        .accounts({
          // @ts-ignore - Causing an issue in Curosr IDE
          vault: otherKeypairVaultPda,
          vaultUsdc: otherKeypairVaultUsdcPda,
          owner: otherOwnerKeypair.publicKey,
          usdcMint: testUsdcMint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })  
        .signers([quartzManagerKeypair])
        .rpc();

      assert.fail(0, 1, "init_user instruction call should have failed");
    } catch (err) {
      expect(err).to.be.instanceOf(Error);
      expect(err.message).to.include(desiredErrorMessage);
    }
  });

  
  it("init_user by user", async () => {
    const {program, otherKeypairVaultUsdcPda, otherKeypairVaultPda, otherOwnerKeypair, testUsdcMint} = testSetup;

    await program.methods
      .initUser()
      .accounts({
        // @ts-ignore - Causing an issue in Cursor IDE
        vault: otherKeypairVaultPda,
        vaultUsdc: otherKeypairVaultUsdcPda,
        owner: otherOwnerKeypair.publicKey,
        usdcMint: testUsdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([otherOwnerKeypair])
      .rpc();
    
    const account = await program.account.vault.fetch(otherKeypairVaultPda);
    expect(account.owner.equals(otherOwnerKeypair.publicKey)).to.be.true;
  });
});
