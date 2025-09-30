#!/bin/bash

cat > README.md << 'EOF'
# Counter Native Solana Program

A simple Solana program written in Rust that maintains a counter stored in a PDA (Program Derived Address).  
The counter can be incremented or decremented by sending instructions.

---

## 📦 Project Setup

### `Cargo.toml`
Make sure your `Cargo.toml` looks like this:

\`\`\`toml
[package]
name = "counter-native-solana"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
borsh = "0.10.3"
solana-program = "2.2.1"
\`\`\`

- `cdylib` → required to compile into `.so` for Solana runtime.  
- `borsh` → used for serializing/deserializing account + instruction data.  
- `solana-program` → provides the Solana program SDK.

---

## 🔨 Build

Compile the program into a deployable `.so`:

\`\`\`bash
cargo build-sbf
\`\`\`

Output files:

\`\`\`
target/deploy/counter_native_solana.so
target/deploy/counter_native_solana-keypair.json
\`\`\`

---

## 🧪 Local Testing

Run a local Solana validator:

\`\`\`bash
solana-test-validator
\`\`\`

Set your CLI to point to it:

\`\`\`bash
solana config set --url localhost
\`\`\`

Fund your local wallet:

\`\`\`bash
solana airdrop 10
\`\`\`

Deploy locally:

\`\`\`bash
solana program deploy target/deploy/counter_native_solana.so
\`\`\`

You’ll get a **program ID** which is used by your client code.

---

## 🌐 Deploy to Devnet

Switch to devnet:

\`\`\`bash
solana config set --url devnet
\`\`\`

Airdrop some SOL on devnet:

\`\`\`bash
solana airdrop 2
\`\`\`

Deploy the program:

\`\`\`bash
solana program deploy target/deploy/counter_native_solana.so
\`\`\`

✅ Deployment Signature:  
\`\`\`
4qb25SizUAWL5Nb7RqjemqdKSLSGi3marYmKHxDWGN1yJdqnQoY7gohKZX3jhr1wcGcJB6hvSXmX1ATo82QLXRMe
\`\`\`

---

## 📊 Program Logic

- Each user has a unique PDA account derived from seeds:  
  \`\`\`
  [ "counter", user_pubkey ]
  \`\`\`
- The PDA stores a `Counter` struct:
  \`\`\`rust
  pub struct Counter {
      pub count: u32,
  }
  \`\`\`
- Instructions:
  - `Increment(u32)` → safely increases the counter.
  - `Decrement(u32)` → safely decreases the counter.

---

## 📡 Interacting with the Program

You can interact using:
- **TypeScript** (`@solana/web3.js` + `borsh`) client.  
- **Rust integration tests** (`solana-program-test`).  
- View logs live with:
  \`\`\`bash
  solana logs
  \`\`\`

---

## 📝 Notes
- Always rebuild (`cargo build-sbf`) after making code changes.  
- On devnet/mainnet, make sure your wallet has enough SOL for deployment and account creation.  
- The **program ID** is fixed at deployment. If you redeploy, you’ll get a new program ID.  
EOF

echo "✅ README.md has been created successfully."
