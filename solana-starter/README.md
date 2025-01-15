# Class 1 - 14/01/25

During this class, we covered some fundamental terminologies and concepts related to Solana, including:
  - Account
  - Program
  - Rent
  - Transaction
  - Compute
  - IDL
  - SPL Token
  - Associated Token Account

### 1. Creating a Mint

This script creates a new mint address using the Solana SPL Token library.
 
File : ```ts/spl_init.ts```

```typescript
import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import wallet from "../wba-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
  try {
    const mint = await createMint(
      connection,
      keypair,
      keypair.publicKey,
      null,
      6,
    );
    console.log(`Mint address: ${mint.toBase58()}`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();

```
Generated Mint Address: [View on Solscan](https://solscan.io/token/CnsyPy8eovZzDbBoJ79VbqKgwEEXcphufE7s3Z5apXrn?cluster=devnet)

### 2. Minting Tokens

This script mints tokens to an associated token account (ATA).

File : ```ts/spl_mint.ts```

```typescript
import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import wallet from "../wba-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

const mint = new PublicKey("CnsyPy8eovZzDbBoJ79VbqKgwEEXcphufE7s3Z5apXrn");

(async () => {
  try {
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    console.log(`Your ata is: ${ata.address.toBase58()}`);
    const mintTx = await mintTo(
      connection,
      keypair,
      mint,
      ata.address,
      keypair.publicKey,
      1n * token_decimals
    );
    console.log(`Your mint txid: ${mintTx}`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();

```
Generated Details :
- Associated Token Account: [View on Solscan](https://solscan.io/account/3UzCVhCfCfLR3moF5jxeKP2vgFdPpZkB2t9U8D5LGLAA?cluster=devnet)
- Mint Transaction ID: Available via logs
