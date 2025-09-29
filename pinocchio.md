# pinocchio: the no-dependency framework for  @solana smart contracts

1. what is pinocchio

pinocchio is a zero-dependency framework for writing solana programs in pure rust. it avoids solana-program, doesn't rely on the standard library, and gives you full control over how your smart contract interacts with the solana runtime.

2. why skip solana-program

the solana-program crate adds helpful abstractions, but it also brings overhead. extra deserialization, hidden allocations, and increased binary size. for programs where compute efficiency or size matters, it's more than you need.

3. how pinocchio works

solana passes your program a raw byte array as input. pinocchio uses this to define its own entrypoint, account types, and system calls. all without needing the solana sdk. you parse exactly what you need, when you need it.

4. entrypoint macro

pinocchio includes an `entrypoint!` macro that sets up your solana program's entrypoint with minimal boilerplate. it handles the program id, accounts array, and instruction data, giving you a clean and direct starting point for logic.

5. lazy entrypoint for more control

if you want to delay parsing and save compute units, pinocchio provides `lazy_program_entrypoint!`. this gives you manual control over when and how to parse accounts or instruction data. useful for lean programs or conditional logic.

6. no allocator mode

you can enforce a no-allocation policy using the `no_allocator!` macro. this ensures that any attempt to allocate memory in your program will fail at runtime. helps you stay fully no_std and avoid hidden costs.

7. when to use pinocchio

use pinocchio when you're building low-level solana programs, working with tight compute budgets, or want to keep your binaries small. it's also useful for protocols doing deep cpi or targeting mev-aware, high-performance flows.

8. when not to use it

if you're prototyping, using anchor for convenience, or don’t mind a bit of overhead, then pinocchio might be overkill. it's a tool for precision, not for speed of development or general-purpose apps.

9. conclusion

pinocchio isn’t trying to replace anchor or solana-program for everyone. it's for builders who care about what every byte and compute unit does. if you're optimizing for performance, safety, and clarity, pinocchio gives you the raw tools to build on solana your way.
