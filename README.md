This is a project to create a WASM-native CPU.

Is it a good idea? Most likely not. Am I going to do it anyways? Yes.

*This project is incomplete.*


This is a little experiment that I'm doing. I'm using **BOOLR** for this and will upload the `.board` file as soon as I get it working.
The below is my brainstorming as I've slowly managed to understand how WASM works and how to implement it. It's probably going to be a good read.

# Step One - Decoding
I'll need to learn how to decode the instructions.

WASM has a _lot_ of instructions. We'll want to learn how we can classify them.

I'll be using [this guide](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#instructions) to help me.

According to the guide, our first challenge is `Control Flow Instructions`.

So, what exactly is a control flow? Why, according to Wikipedia: **the order in which individual statements, instructions or function calls of an imperative program are executed or evaluated.
That's a mouthful. What does it actually mean? Well, it basically just means that these operations control the order of execution of operations in WASM.

The first instruction we have is the **Block** instruction, with opcode `0x02`, or `00000010` in binary. It will push an entry onto the control-flow stack.

This line confuses me. What exactly *is* the control flow stack? 
To figure this out, I've realized that I can't just jump into this and expect this to work. So we're going to be taking a slight detour to and start reading.

## Instruction families
At the moment, WASM has 12 instruction families. The reference manual lists them all in detail, so I'm not going to cover all of them.

The family of interest is family `Q`, or the `Control-Flow Barrier Instruction Family`.
The long and short of it is that all of these operations alter the flow of the program so that the execution does *not* proceed to the next instruction(but rather, to a different instruction, or no instruction at all).
These are important for logic and *branching*.

## Control Flow Instructions 

In other words, the `block` will help us with the flow of the program. We may not understand this now, but we'll understand it more later. It also takes in a [block signature type](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#block-signature-types)
Also, when I said it would push an "entry". The correct term is an *unbound label*, where a label is either unbound or bound to a specific position. (See [this](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#labels))
Lastly, the block needs to have an `end`(which we'll cover later) to bind it and pop it from the stack.

The next instruction is a *loop*, or opcode `0x03`. This instruction *binds* a label to the current position, and pushes that entry onto the control-flow stack.
Like block, it takes a block signature type. It *also* needs an end. The loop does not actually perform a loop. Instead, it adds a label which can *later* be used by a branch for looping.

Speaking of branches, up next is the unconditional branch. It will *branch* the program according to the control-flow stack entry a certain `depth` away from the top, where a `depth` is a `varuint32`.

### Branching
Now would be a good time to discuss branching. Branching is when the program jumps to a label in the control-flow stack. If the label is bound, it goes to the bound position.
If not, it scans forward until the label *is* bound(by just "executing" `block`, `loop`, and `end` instructions). It sets the position to that position.
Finally, all the control-flow stack entries are popped until that position.

**TLDR - If you've ever worked with assembly or done some research on CPUs, it's the equivalent of a jump.**


