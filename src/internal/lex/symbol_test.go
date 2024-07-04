package lex_test

import (
	"testing"

	"github.com/winstonpuckett/welang/src/internal/lex"
)

/*
Given the symbol and nothing else
When we lex
We should increment by 1
*/
func TestIncrementsWithOneChar(t *testing.T) {
	for _, tc := range symbolCases {
		t.Run(tc.source, func(t *testing.T) {
			got := tc.fn(tc.source, 0)

			if got != 1 {
				t.Errorf(tc.source + " did not increment.")
			}
		})
	}
}

/*
Given a random char and the char we are looking for after
When we lex
We should not increment
*/
func TestNotIncrementsWhenNext(t *testing.T) {
	for _, tc := range symbolCases {
		t.Run(tc.source, func(t *testing.T) {
			got := tc.fn(nonSymbol+tc.source, 0)

			if got != 0 {
				t.Errorf(tc.source + " did increment.")
			}
		})
	}
}

/*
Given the symbol in the middle and we index into the middle
When we lex
We should increment by 1
*/
func TestIncrementsWithCharInMiddle(t *testing.T) {
	for _, tc := range symbolCases {
		t.Run(tc.source, func(t *testing.T) {
			got := tc.fn(nonSymbol+tc.source+nonSymbol, 1)

			if got != 2 {
				t.Errorf(tc.source + " did not increment.")
			}
		})
	}
}

/*
Given two symbols
When we lex
We should increment by only 1
*/
func TestIncrementsWithTwoSymbols(t *testing.T) {
	for _, tc := range symbolCases {
		t.Run(tc.source, func(t *testing.T) {
			got := tc.fn(tc.source+tc.source, 0)

			if got != 1 {
				t.Errorf(tc.source + " did not increment.")
			}
		})
	}
}

/*
Given a blank string
When we lex
Then we should not increment
*/
func TestNotIncrementsWithBlank(t *testing.T) {
	for _, tc := range symbolCases {
		t.Run(tc.source, func(t *testing.T) {
			got := tc.fn("", 0)

			if got != 0 {
				t.Errorf(tc.source + " did increment.")
			}
		})
	}
}

// Any character that isn't a valid symbol.
var nonSymbol = "a"

// How each symbol maps to its function.
var symbolCases = []struct {
	source string
	fn     func(string, int) int
}{
	{"{", lex.ContextStart},
	{"}", lex.ContextEnd},
	{"[", lex.ArrayStart},
	{"]", lex.ArrayEnd},
	{"(", lex.TypeStart},
	{")", lex.TypeEnd},
	{",", lex.Separator},
	{"'", lex.TypeLiteral},
	{"|", lex.Pipe},
	{";", lex.FunctionEnd},
	{"_", lex.Discard},
}
