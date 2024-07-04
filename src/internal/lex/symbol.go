package lex

/*
symbols:
{}
[]
()
|
;
'
_

patterns:
numbers 0,-1,0x0,0b0,0o0,0.0
identifiers

keywords:
fn
alias
use
identity
*/

func ContextStart(source string, start int) int {
	return symbol(source, start, '{')
}

func ContextEnd(source string, start int) int {
	return symbol(source, start, '}')
}

func ArrayStart(source string, start int) int {
	return symbol(source, start, '[')
}

func ArrayEnd(source string, start int) int {
	return symbol(source, start, ']')
}

func TypeStart(source string, start int) int {
	return symbol(source, start, '(')
}

func TypeEnd(source string, start int) int {
	return symbol(source, start, ')')
}

func Separator(source string, start int) int {
	return symbol(source, start, ',')
}

func TypeLiteral(source string, start int) int {
	return symbol(source, start, '\'')
}

func Pipe(source string, start int) int {
	return symbol(source, start, '|')
}

func FunctionEnd(source string, start int) int {
	return symbol(source, start, ';')
}

func Discard(source string, start int) int {
	return symbol(source, start, '_')
}

func symbol(source string, start int, symbol byte) int {
	if len(source) <= start {
		return start
	}

	if source[start] == symbol {
		return start + 1
	}
	return start
}
