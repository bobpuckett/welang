# Primitives

## Function

A list of steps to execute.

```welang
fn identifier inType, outType
    steps | steps
    steps
    steps;
```

## Unsigned integer

An arbitrary set of bits, specified in binary.

```welang
# type
uX (u8, u16, u1)

# value
0b0110
123
```

## Signed integer

A number between a certain range, specified in decimal.

```welang
# type

# a number between 20-40, inclusive
i20:40

# a number between negative 100 and postitive 100, inclusive
i-100:100

# value
-10
100
0
```

## Context

A key-value-pair list where each key is an identifier.

```welang
# type
{
    key: u16,
    key2: i0:10
}

# value
{
    key: 7,
    key2: 10
}
```

## Type alias

A type restraint which restricts an input to satisfy the specified type value.

```welang
# format: alias name value

alias age i0:150

alias person {
    age: age,
    weight: u8
}
```

## Type identity

A type restraint which restricts an input to claim it is the type specified

```welang
identity employeeId u16

identity employee {
    id: employeeId,
    jobSatisfaction: i0:100
}
```

# Functions

Each step in a function can reference the output of the previous step and all imported constructs available. 