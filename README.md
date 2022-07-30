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
*Thanks, documentation.*

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

## Analyzing some WASM
Even though we've only looked at the bare basics, I feel like maybe we should take a look at some compiled WASM so we can get a better understanding of what happens under the hood.

Now, let's steal an example from [this](https://rsms.me/wasm-intro) article.

So we start with a very simple halving program. Here's how it would look in C:
```c
int half(int x) {
  return x/2;
}
```

Here's a portion of the `.wat` result:
```wat
get_local 0
i32.const 2
i32.div_u
```

Let's break it down:

`get_local 0` just pushes paramter `0` onto the stack.

`i32.const 2` pushes the number 2 onto the stack.

`i32.div_u` does (unsigned)division. Unsigned just means that we're working only with positive numbers(instead of two's complement).

Lastly, `end` just ends the function. Well, that's nice. Actually working with integers in WASM is pretty convinient, because it has 29 different integer operations. That *won't* be fun to implement.

Here's a more complex *factorial* program:
```c
int64 factorial(int64 n) {
  return (n == 0) ? 1
                  : n * factorial(n - 1)
}
```

That's some normal C code I stole from the article. Here's the resulting WAT:
```wat
(module
  (type $t0 (func (param f64) (result f64)))
  (func $fac (export "fac") (type $t0) (param $p0 f64) (result f64)
    (if $I0 (result f64)
      (f64.lt
        (local.get $p0)
        (f64.const 0x1p+0 (;=1;)))
      (then
        (f64.const 0x1p+0 (;=1;)))
      (else
        (f64.mul
          (local.get $p0)
          (call $fac
            (f64.sub
              (local.get $p0)
              (f64.const 0x1p+0 (;=1;)))))))))
```

*Note, this is what I get from wasm2wat*

So this is our *module*. This WASM module would be imported into a main WASM, which would probably run a little program(or, eventually, an operating system) directly on the hardware.

Firstly, we can see our *type  section*. It has one `$t0` type, which defines a function which takes an and outputs an `f64` float.
We then define said function *using* the type(the *signature*).
After that, we have an `if`. Under the hood, the `if` adds a control-flow stack entry to the program which contains an *unbound label*. If it's condition is false, it branches according to that label.

Remember how branching works? It will skip ahead until it finds something that *defines* the label, which would happen to be an *end*, or, in our case, and `else`.
The `then` statement is *useless*, it's only there to differentiate from the `if` contents and what to actually do. In reality, WASM is a stack machine, so that whole part is more neatly represented as this:
```wat
  local.get $p0
  f64.const 1
  f64.lt
  if (result f64)
    f64.const 1
  else
    local.get $p0
    local.get $p0
    f64.const 1
    f64.sub
    call $fac
    f64.mul
```

Isn't that so much nicer? I think so. You may have some confusion as to what goes on here. So let me clarify.
We get the first parameter, and put it on the stack. We then add an `f64` 1 onto the stack. The `f64.lt` operation pops the first value off the stack, and then pops the second value off to see if it's less than the first value.
It then adds an `i32` of either `0` or `1`(our fake boolean) onto the stack. The `if` then checks if this is true. If it is true, then our value is 0, and it sets `(result f64)` to `1`.
Otherwise, it adds our paramter onto the stack *twice*, and then another one. It then subtracts them, popping the top two values off the stack, subtracting them(with the second value first), and then putting the result back onto the stack.
We then call the `fac`torial function on this number in the stack, which leaves us with two things on the (program) stack: our original paramater and the result of the factorial. We multiply these two things together and set `(result f64)` to this value.
Lastly, we call `end` to end the funciton.

Now, are you ready for something a *lot* more challenging? Probably not. But I don't care!
Here's the *compiled* WASM bytecode:
```
00 61 73 6D 01 00 00 00 01 06 01 60 01 7C 01 7C
03 02 01 00 07 07 01 03 66 61 63 00 00 0A 2E 01
2C 00 20 00 44 00 00 00 00 00 00 F0 3F 63 04 7C
44 00 00 00 00 00 00 F0 3F 05 20 00 20 00 44 00
00 00 00 00 00 F0 3F A1 10 00 A2 0B 0B 00 12 04
6E 61 6D 65 01 06 01 00 03 66 61 63 02 03 01 00
00
```

*Note: this isn't the *actual* binary. It's the binary, written in hex because it's a lot more consise and way easier to understand.*

Kind of scary, isn't it? Luckily, [this amazing tool](https://wasdk.github.io/wasmcodeexplorer/) exists, and it's a *lifesaver*.

I just loaded up the WASM file and immediately it decompiled it and highlighted parts of the binary for me.
It was *incredibly* helpful and I implore *you* to check it out!

So, now I can go bit-by-bit into that *scary* byte code.

The first part, `00 61 73 6D 01 00 00 00 ` just lets us know our version of WASM(version `1`) and our "magic number"(I'll explain *later*). Then, we have `01 06`, which initiates our *type section*.
The following `01 60 01 7C 01 7C` tells us that both our input and output are floats(`-0x04`). We then have a function section, which I haven't covered yet. TLDR, it defines all the module's functions and gives them indexes(in the bytecode, this is `03 02 01 00`).
After that, we start the *export* section with `07 07`. The following `01 03 66 61 63 00 00` exports *amazing* `fac` function(`66 61 63` are it's ascii codes, `00` ends the string and then `00` is it's ID). This line correlates to `(export "fac" (func $func0))`
`0A 2E` is the best part. It defines the *code section*. Now we can start coding. (`0x0a` is the code section opcode, and it is followed by the number of bytes. In this case, that is 46, or `2E`.)

Now, we have the function body, defined by `01 2C 00`. We can use our intuition to decode this. The `0x01` begins the function definition. The following `2C`, is *44*, and tells us that the function body itself is 44 bytes(this checks out).
Lastly, the `00` is a list of local paramaters, which in this case is just 0(recall the `local.get 0`).

Now, the actual code. We start with the [local.get](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#get-local), which adds a local paramater to the stack.
In the code, we see this as `20 00`, with `0x20` being the opcode for `local.get`. This is followed by the [f64.const](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#constant) instruction, which adds a constant onto the stack.
Note that since we are dealing with floats(an `f64`), this is quite long: `44 00 00 00 00 00 00 F0 3F`. This is equivalent to `f64.const 1`.
We then perform `f64.lt`, which is just opcode `0x63`. Following that, we have an [if statement](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#if), which binds the `(result f64)`.
Recall that an `f64` is represented as `7C`, giving us `04 7C`. We then have a line to add an `f64.const 1`, which adds 1 to the stack, and is the same `44 00 00 00 00 00 00 F0 3F` from before.
We then have an [else](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#else), which would bind the `if`'s unbound label on the control flow stack.
Following the else, we have `20 00`, which, if you'll remember, means to get the first local variable. Then, we do it again, so another `20 00`. We have yet *another* `44 00 00 00 00 00 00 F0 3F`, because adding 1s to the stack is quite common in this program.
We then call [subtract](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#floating-point-subtract), which for an `f64` is opcode `0xa1`, giving us the `A1` in the code.
Remember back in the export, where we set the function id to zero? We can call that function now. We use the [call function](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#call), which has opcode `0x10`, to call the function again(recursion!). WASM natively supports recursion, which means this chip will have to too. These function actually have a special call stack. You can learn more about calling [here](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#calling).
Lastly, we call a [floating point multiply](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#floating-point-multiply) on the stack, using the `A2` operation. This final result is passed in as the return value(if the function was called, which it most likely was, it'll be put onto the stack.)

Now, at this point in time, I'd written very little code. So I thought it would be a good time to start writing something now. This *fairly basic* program implements a lot of core functionality of WASM, therefore I want to start by implementing enough instructions for *this specific program* to run.

So let's give it a go!

I started by defining our control-flow, program, and call stacks. If we need more stacks, we'll add them in later. A `Vec<u8>` works perfectly for the cause. Next, we check the magic number. This is just the `\0asm` at the beginning of the file. I like to call this our `sus detector`.
