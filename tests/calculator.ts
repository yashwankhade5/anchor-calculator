import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { assert } from "chai";

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.calculator as Program<Calculator>;
const newAccount= anchor.web3.Keypair.generate()
  it("Is initialized!", async () => {
    // Add your test here.


const tx = await program.methods
  .init(12)
  .accounts({
    signer: anchor.getProvider().wallet.publicKey,
    account: newAccount.publicKey,
  })
  .signers([newAccount])
  .rpc();

   
    
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey)
    assert(account.count == 12)
  });


  it("double",async ()=>{
    let previous_count = await program.account.dataShape.fetch(newAccount.publicKey)
    const tx = await program.methods.double()
    .accounts({
      signer:anchor.getProvider().wallet.publicKey,
      account:newAccount.publicKey
    })
    .rpc()

     console.log("Your transaction signature", tx);
 const new_count = await program.account.dataShape.fetch(newAccount.publicKey)
 

     assert.equal(new_count.count,previous_count.count*2)
  })

  it("add ",async () => {
    const previous_count  = await program.account.dataShape.fetch(newAccount.publicKey)

    const tx = await program.methods.add(12)
    .accounts({
      signer:anchor.getProvider().wallet.publicKey,
      account:newAccount.publicKey
    })
    .rpc()
    console.log("tx id gone through",tx)
    const new_count = await program.account.dataShape.fetch(newAccount.publicKey)
    console.log(new_count.count)
    console.log(previous_count.count+12)

    assert.equal(previous_count.count+12,new_count.count)
  })
  
  it("half",async()=>{
    const previous_count  = await program.account.dataShape.fetch(newAccount.publicKey)
    const tx = await program
  })
});
