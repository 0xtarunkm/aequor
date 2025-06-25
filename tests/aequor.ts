import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Aequor } from '../target/types/aequor';
import { PublicKey } from '@solana/web3.js';
import { createMint } from '@solana/spl-token';
import assert from 'assert';
import { BN } from 'bn.js';

describe('aequor', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.aequor as Program<Aequor>;
  const provider = anchor.AnchorProvider.env();

  let configKeypair: anchor.web3.Keypair;
  let configPubkey: PublicKey;
  let feeTierKeypair: anchor.web3.Keypair;

  it('Initializes the protocol config', async () => {
    configKeypair = anchor.web3.Keypair.generate();
    const feeAuthority = provider.wallet.publicKey;
    const collectFeeAuthority = provider.wallet.publicKey;
    const protocolFeeRate = 100; // 1% as an example

    const tx = await program.methods
      .initializeConfig(feeAuthority, collectFeeAuthority, protocolFeeRate)
      .accountsStrict({
        signer: provider.wallet.publicKey,
        config: configKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([configKeypair])
      .rpc();

    console.log('Config transaction signature', tx);

    const configAccount = await program.account.aequorConfig.fetch(
      configKeypair.publicKey
    );
    configPubkey = configKeypair.publicKey;

    assert.ok(configAccount.feeAuthority.equals(feeAuthority));
    assert.ok(configAccount.collectFeeAuthority.equals(collectFeeAuthority));
    assert.equal(configAccount.protocolFeeRate, protocolFeeRate);
  });

  it('Initializes a fee tier', async () => {
    feeTierKeypair = anchor.web3.Keypair.generate();
    const tickSpacing = 1;
    const defaultFeeRate = 30; // 0.3% as default fee

    const tx = await program.methods
      .initializeFeeTier(tickSpacing, defaultFeeRate)
      .accountsStrict({
        signer: provider.wallet.publicKey,
        feeTier: feeTierKeypair.publicKey,
        aequorsConfig: configPubkey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([feeTierKeypair])
      .rpc();

    console.log('Fee tier transaction signature', tx);

    const feeTierAccount = await program.account.feeTier.fetch(
      feeTierKeypair.publicKey
    );
    assert.equal(feeTierAccount.tickSpacing, tickSpacing);
    assert.equal(feeTierAccount.defaultFeeRate, defaultFeeRate);
  });

  it('Initializes a pool', async () => {
    const mintA = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    console.log('mint a: ', mintA);

    const mintB = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    console.log('mint b: ', mintB);

    const tickSpacing = 1;

    const [aequorPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('aequor'),
        configPubkey.toBuffer(),
        mintA.toBuffer(),
        mintB.toBuffer(),
        Buffer.from(new Uint16Array([tickSpacing]).buffer, 0, 2),
      ],
      program.programId
    );

    console.log('aequor PDA: ', aequorPDA);

    const [vaultA] = PublicKey.findProgramAddressSync(
      [Buffer.from('vault_a'), aequorPDA.toBuffer()],
      program.programId
    );

    console.log('vault A: ', vaultA);

    const [vaultB] = PublicKey.findProgramAddressSync(
      [Buffer.from('vault_b'), aequorPDA.toBuffer()],
      program.programId
    );

    console.log('vault B: ', vaultB);

    const initialSqrtPrice = new BN('79228162514264337593543950336');

    const tx = await program.methods
      .initializePool(tickSpacing, initialSqrtPrice)
      .accountsStrict({
        signer: provider.wallet.publicKey,
        mintA: mintA,
        mintB: mintB,
        vaultA: vaultA,
        vaultB: vaultB,
        aequorsConfig: configPubkey,
        aequor: aequorPDA,
        feeTier: feeTierKeypair.publicKey,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log('Pool initialization transaction signature', tx);

    const poolAccount = await program.account.aequor.fetch(aequorPDA);
    assert.ok(poolAccount.mintA.equals(mintA));
    assert.ok(poolAccount.mintB.equals(mintB));
    assert.equal(poolAccount.tickSpacing, tickSpacing);
    assert.equal(poolAccount.sqrtPrice.toString(), initialSqrtPrice.toString());
  });
});
