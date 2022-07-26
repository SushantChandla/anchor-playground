import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Playground } from "../target/types/playground";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";

export const encodeSeedString = (seedString: string) =>
	Buffer.from(anchor.utils.bytes.utf8.encode(seedString));

describe("playground", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Playground as Program<Playground>;

  it("Is initialized!", async () => {
    // Add your test here.
    const seeds =[encodeSeedString("seed")];
    const globalState = await PublicKey.findProgramAddress(seeds,program.programId);

    const tx = await program.methods.initialize().accounts({
      globalState: globalState[0],
      owner: program.provider.publicKey,
      rent: SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
