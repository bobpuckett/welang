# constructs

fn name (in, out)
    step2 step1
    | step3;

> Function Type: 'fn(in,out)

alias name definition
identity name definition

# literal | type pairs

unsigned integer
    12345678
    'lowerbound:upperbound '0:127

signed integer
    -123345678
    'lowerbound:upperbound '-127:256

real
    123.456
    'lowerbound:upperbound:precision '-1:254:3

other literals
    binary: 0b1110
    octal: 0o017
    hex: 0xF02

context
    { key: value }
    '{ identifier: type } '{ name: '-127:256 }

map
    { key: value }
    '{ primitaveKeyType: valueType } '{ u16, u32 }

array
    (type)[ 0, 1, 2 ]
    '[type] '[u8]

tuple
    {{ value, value, value }}
    '{{ type, type, type }} '{{ u8, u16, r32 }}

# package management

use path
use path.subpath

> This means no spaces in filenames.