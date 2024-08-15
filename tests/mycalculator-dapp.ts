import assert from "assert";
import * as anchor from "@project-serum/anchor";
const { SystemProgram } = anchor.web3;

describe('mycalculator-dapp', () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();

  const program = anchor.workspace.MycalculatorDapp;

  it('Creates a calculator', async () => {

    await program.rpc.create("Welcome to Solana", {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [calculator],  
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);

    assert.ok(account.greeting === "Welcome to Solana");
  });
  it ('Adds two numbers', async () => {
    await program.rpc.add(new anchor.BN(5), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
      signers: [calculator],
    });
  })
});
