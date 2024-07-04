package lex_test

import (
	"testing"

	"github.com/winstonpuckett/welang/src/internal/lex"
)

/*
Given the keyword and nothing else
When we lex
We should increment by the keyword character count
*/
func TestIncrementsWithKeyword(t *testing.T) {
	for _, tc := range keywordCases {
		t.Run(tc.source, func(t *testing.T) {
			got := tc.fn(tc.source, 0)

			if got != len(tc.source) {
				t.Errorf("%s returned %d instead of %d",
					tc.source,
					got,
					len(tc.source))
			}
		})
	}
}

/*
Given the keyword and [a-zA-Z0-9_] after
When we lex
We should not increment
*/
func TestKeywordNotParsedWithSuffix(t *testing.T) {
	for _, tc := range keywordCases {
		for _, c := range everySymbollikeChar {
			t.Run(tc.source, func(t *testing.T) {
				got := tc.fn(tc.source+c, 0)

				if got != 0 {
					t.Errorf("%s returned %d instead of %d",
						tc.source,
						got,
						0)
				}
			})
		}
	}
}

/*
Given the previous char was [a-zA-Z0-9_] and the keyword
When we lex
We should not increment
And we should panic
*/
func TestKeywordNotParsedWithPrefix(t *testing.T) {
	for _, tc := range keywordCases {
		for _, c := range everySymbollikeChar {
			t.Run(tc.source, func(t *testing.T) {
				t.Logf("c: %s, source: %s", c, tc.source)
				got := tc.fn(c+tc.source, 1)

				if got != 1 {
					t.Errorf("%s returned %d instead of %d",
						tc.source,
						got,
						0)
				}
			})
		}
	}
}

/*
Given a word that starts as a keyword but has an incorrect character
When we lex
Then we should not increment
*/
func TestKeywordNotParsedWithIncorrectChar(t *testing.T) {
	for _, tc := range keywordCases {
		t.Run(tc.source, func(t *testing.T) {
			for i := 0; i < len(tc.source); i++ {
				incorrectSource := tc.source[:1] + "!" + tc.source[1:]
				got := tc.fn(incorrectSource, 0)

				if got != 0 {
					t.Errorf("%s returned %d instead of %d",
						tc.source,
						got,
						0)
				}
			}
		})
	}
}

/*
Given a keyword with a space suffix
When we lex
We should increment by the keyword character count
*/
func TestKeywordParsedWithSpaceSuffix(t *testing.T) {
	for _, tc := range keywordCases {
		t.Run(tc.source, func(t *testing.T) {
			got := tc.fn(tc.source+" ", 0)

			if got != len(tc.source) {
				t.Errorf("%s returned %d instead of %d",
					tc.source,
					got,
					len(tc.source))
			}
		})
	}
}

// Could be anything matching [a-zA-Z0-9_]
var everySymbollikeChar = []string{
	"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
	"p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D",
	"E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
	"T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4", "5", "6", "7",
	"8", "9", "_"}

var keywordCases = []struct {
	source string
	fn     func(string, int) int
}{
	{
		source: "use",
		fn:     lex.UseKeyword,
	},
	{
		source: "alias",
		fn:     lex.AliasKeyword,
	},
	{
		source: "identity",
		fn:     lex.IdentityKeyword,
	},
	{
		source: "fn",
		fn:     lex.FunctionKeyword,
	},
}
