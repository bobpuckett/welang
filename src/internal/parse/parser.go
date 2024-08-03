package parse

import (
	"fmt"

	"github.com/winstonpuckett/welang/src/internal/lex"
)

type parser func(string, int) *Node
type lexer func(string, int) int
type parseResult func(string, int) (*Node, int, error)
type lexerPredicate func(*Node, string, int) (int, error)

func Parse(source string, start int) *Node {
	parsers := []parser{}
	next := start

	for _, p := range parsers {
		parsed := p(source, next)
		if parsed != nil {
			return parsed
		}
	}

	// TODO: Handle case
	return nil
}

func parseFunction(source string, start int) (*Node, int) {
	function, pos, err := parseBuilder(source, start,
		exists(lex.FunctionKeyword),
		exists(lex.TypeStart),
		one(lex.Identifier, func(node *Node, value string) { node.InType = Type{Claim: value} }),
		one(lex.Identifier, func(node *Node, value string) { node.OutType = Type{Claim: value} }),
		exists(lex.TypeEnd),
		functionBody,
		exists(lex.FunctionEnd),
	)

	if err != nil {
		return nil, start
	}

	return function, pos
}

// An accumulator which accepts a list of functions which accumulate toward the
// passed in node using the source and start position.
// It returns the resulting node, the new position, and an error if one occurred.
func parseBuilder(source string, start int, predicates ...lexerPredicate) (*Node, int, error) {
	node := &Node{}
	pos := start

	for _, word := range predicates {
		next, err := word(node, source, pos)
		if err != nil {
			return nil, start, err
		}

		pos = next
	}

	return node, pos, nil
}

func one(lexer lexer, handler func(*Node, string)) func(*Node, string, int) (int, error) {
	return func(node *Node, source string, start int) (int, error) {
		pos := consumeWhitespace(source, start)
		pos = lexer(source, start)
		if pos == start {
			return start, fmt.Errorf("expected to lex one, but couldn't find it")
		}

		handler(node, source[start:pos])
		return start, nil
	}
}

func exists(lexer lexer) func(*Node, string, int) (int, error) {
	return func(node *Node, source string, start int) (int, error) {
		pos := consumeWhitespace(source, start)
		pos = lexer(source, start)
		if pos == start {
			return start, fmt.Errorf("expected to lex one, but couldn't find it")
		}

		return pos, nil
	}
}

func firstMatch(predicates ...lexerPredicate) func(*Node, string, int) (int, error) {
	return func(node *Node, source string, start int) (int, error) {
		for _, pred := range predicates {
			pos, err := pred(node, source, start)
			if err == nil {
				return pos, nil
			}
		}

		return start, fmt.Errorf("no predicates matched")
	}
}

func functionBody(main *Node, source string, start int) (int, error) {
	var steps *Node
	var currentChain *Node

	pos := consumeWhitespace(source, start)
	for {
		if lex.FunctionEnd(source, pos) != pos {
			break
		}

	}

	return pos, nil
}

func consumeWhitespace(source string, start int) int {
	for start < len(source) && isWhitespace(source[start]) {
		start++
	}

	return start
}

func isWhitespace(char byte) bool {
	return char == ' ' || char == '\t' || char == '\n' || char == '\r'
}
