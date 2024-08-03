# Phases

There will be two phases to the compiler

1. Symantec analysis: Create a recursive map of constructs based on the source files. Make sure there are no unexpected symbols given the contexts.
1. Cartographic analysis: Finalize the types within the map. Ensure nothing is missing, passing the wrong type or accepting the wrong type.
1. Path analysis: Starting from the bottom of the main function, create a single pathway which the code will follow, inlining all function calls, creating a memory allocation and de-allocation for each statement. Ensure we know exactly where all memory is de-allocated.
1. Composition analysis: Find all duplicate sections of code and extract functions based on a threshold of size over time.
1. Lowering: Writing the resulting program to an executable.