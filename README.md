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

Now, I've been glossing over quite a bit. For one thing, I haven't even *touched* signatures. So let's touch some signatures.

### Touching signatures
So WASM has this neat concept of types. Actually, WASM has a neat concept of a lot of things, which make it very nice and type safe and **extremely annoying to port**, especially to an actual CPU.
But back to types. So there's two types of types(that's right, type types). Currently, we're going to be looking at language types. Language types are used to "describe runtime values and language constructs".
In other words, important stuff.
They also have a *type encoding*, which is a type of language type(so a type type type). Here's a neat table which showcases them:

| Name      | Binary Encoding |
| --------- | --------------- |
| `i32`     | `-0x01`         |
| `i64`     | `-0x02`         |
| `f32`     | `-0x03`         |
| `f64`     | `-0x04`         |
| `funcref` | `-0x10`         |
| `func`    | `-0x20`         |
| `void`    | `-0x40`         |

*A neat table, stolen directly from the documentation*

Now for the actual language types:
- Value Types
- Table Element Types
- *Signature Types*
- Block Signature Types
We're going to be focusing on signature types right now. Signature types are, in the end, really just any type defined in the Type Section, which is a section in the code that defines the types.
This is getting confusing, isn't it?

The Type Section(denoted by opcode `0x01`) is an array of function signatures. Yeah, every thing comes back full circle.
A *function signature* consists of a `form`, which is a type `signature type`.
Also, if the `form` is a `func`, which it is required to be, we get these two extra fields:
- `params`
  - An `array` of `value type`s. Consists of function paramters.
- `returns`
  - An array of `value type`s. Consists of the function's returns.
The function must have a `returns` with *at least* one element.
A `value type` is another `Language Type` which is the type of the input and output values of instructions once they are executed.

Was that confusing? Probably. In short, we use signatures to define the inputs and outputs of instructions.

Back to branches. The unconditional branch uses a signature of `block_arity` for input *and* output.
The `block_arity` type really just tells you how many inputs and outputs are in the branching control-flow's stack entry.

Conditional branches have a signature of `block_arity` *and* a `condition` of type `i32`. Note that in WASM, `i32` is used for booleans, with `0` being false and `1` being true.

It's the same as an unconditional branch(using `$depth` to branch in the control-flow stack), but only if `condition` is true.
Otherwise, it just "falls through" and lets execution proceed as normal.

Both unconditional and conditional branches return the value of their `block_arity` operands.
