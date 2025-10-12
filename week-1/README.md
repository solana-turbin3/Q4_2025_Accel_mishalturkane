# Week1
## class - 1 {Token Extensions on Solana}
## 🧩 What Are Token Extensions in Solana

That’s a good one — and a really important one if you’re building serious stuff on Solana right now.

So, **Token Extensions** (a.k.a. the **Token-2022 program**) are Solana’s way of evolving beyond the original `spl-token` standard. The first version (`spl-token`) was simple and lightweight — perfect for fungible tokens like SPL coins or basic NFTs. But as Solana matured, devs wanted more advanced token features **without reinventing or redeploying custom token programs every time**.

Let’s break it down cleanly:

---

### 🧩 What Token Extensions Are
Token extensions are **modular add-ons** baked into the `spl-token-2022` program. They extend the base token functionality with optional features — kind of like “plugins” for your token. You can turn them on during mint creation (they’re immutable afterward).

You still get all the usual SPL behavior — minting, transferring, freezing, burning — but with extra control layers.

---

### 🚀 Why They’re Used (and Loved)

Here’s what you get and *why* it matters:

1. **Transfer fees & royalties**
   You can define automatic fees on transfers (e.g. 0.5% per transaction) — useful for RWA protocols, DAOs, or projects needing built-in revenue mechanisms.

2. **Interest-bearing / Transfer hooks**
   Tokens can trigger programmatic actions when moved — imagine on-chain compliance checks, dynamic pricing, or wrapping logic on transfer.

3. **Confidential transfers (Zero-knowledge style)**
   Solana’s version of privacy: hide balances and amounts while still proving validity cryptographically. Great for RWA or private DeFi.

4. **Metadata & memo fields**
   You can embed structured metadata (like off-chain links, token purpose, etc.) directly into the token’s account structure.

5. **Permanent delegate / freezing authority**
   You can predefine authorities who can freeze, claw back, or manage compliance logic — vital for regulatory assets like RWAs or stablecoins.

6. **Close authority**
   Lets someone close token accounts and recover rent — helpful for wallet cleanup or dApps with many ephemeral accounts.

7. **Default Account State**
   Control whether accounts start frozen, uninitialized, etc., improving compliance and user flow.

---

### 🏗 Example Use Case
Let’s say you’re building a **tokenized real-world asset (RWA)** — like tokenized property shares or treasury bills.
You’d probably want:

- **Transfer fees or restrictions** (for compliance)
- **Freeze authority** (to handle KYC or fraud cases)
- **Metadata extension** (to link off-chain asset data)
- **Confidential transfers** (so holdings aren’t public)

All of that can be handled **natively** using Token-2022 extensions — no need to reinvent token logic in Anchor or deploy custom contracts.

---

### ⚙️ Developer Tip
When you initialize a mint using Token Extensions, you just use:
```bash
spl-token-2022 create-token
```
instead of the classic:
```
spl-token create-token
```
Then, you can specify extensions like:
```
spl-token-2022 create-token --transfer-fee 10 1000

```
