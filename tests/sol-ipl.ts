import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolIpl } from "../target/types/sol_ipl";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
describe("sol-ipl", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as NodeWallet;
  const program = anchor.workspace.SolIpl as Program<SolIpl>;

  it("Is initialized!", async () => {
    // Add your test here.
    try {
      const tx = await program.methods
        .createAggregator()
        .accounts({
          aggregatorFeed: new anchor.web3.PublicKey(
            "6Xqt47Hkpr62sUDkBRfKuTuejE6BNBAYFJb1vVyjLUv8"
          ),
          authority: wallet.publicKey,
        })
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("Error: ", error);
    }
  });
});
