# Compile Targets

Effort is ranked on a fibonacci scale, starting with targeting a higher level language as the easiest.

# Option 1: Assembly

Effort: 8

Benefits:
- Easier to learn than LLVM or GCC
- Best performance

Tradeoffs:
- Have to write things like a hashmap
- Will have to have an external compiler?

# Option 2: LLVM

Effort: 13

Benefits:
- Handles compilation, etc. itself

Tradeoffs:
- Esoteric
- Every language written with LLVM seems to have a major drawback (Rusts compile times for instance)

# Option 3: GCC

Effort: 13

Benefits:
- Handles compilation
- Incredibly fast

Tradeoffs:
- Esoteric
- May have to write *in* GIMPLE or similar

# Option 4: WASM

Effort: 5

Benefits:
- Surprisingly less esoteric
- Targets are on the rise

Tradeoffs:
- Performance
- Very narrow deployment strategies

# Option 5: Just interpret

Effort: 2

Benefits:
- No compilation considerations

Tradeoffs:
- Performance is locked to the source language
- Won't be considered a "real" language

# Option 6: Higher Level Language

Effort: 1

Benefits:
- No learning curve
- Fewer opportunities for bugs because of familiarity and "validatability"

Tradeoffs:
- Performance locked to target language
- Won't be considered a "real" language

# Decision

The major question we need to answer right now is "but will it be as fun as we think?"

It's fun to write less code while having a more maintainable codebase. We need to get that right before moving on to other considerations.

That being said, we also need to get 100% compiled memory management right.

Because of this, picking the *easiest* compile target instead of the *most efficient* compile target will allow us to validate our best aims.

The easiest compile target is JavaScript. Let's use that and worry about writing our own hashmap after we know the language is fun.