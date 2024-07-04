package lex_test

import (
	"testing"

	"github.com/winstonpuckett/welang/src/internal/lex"
)

/*
Given an identifier with a prefix before it
When we lex
We should parse the identifier
*/
func TestIdentifierWithPrefix(t *testing.T) {
	t.Run("identifier with prefix", func(t *testing.T) {
		newStart := lex.Identifier("|anyIdentifier", 1)
		if newStart != 14 {
			t.Errorf("identifier with prefix returned %d instead of %d",
				newStart,
				14)
		}
	})
}

/*
Given an identifier with a suffix after it
When we lex
We should parse the identifier
*/
func TestIdentifierWithSuffix(t *testing.T) {
	t.Run("identifier with suffix", func(t *testing.T) {
		newStart := lex.Identifier("anyIdentifier|", 0)
		if newStart != 13 {
			t.Errorf("identifier with suffix returned %d instead of %d",
				newStart,
				13)
		}
	})
}

/*
Given something which is not an identifier
When we lex
We should not increment
*/
func TestIdentifierWithNonIdentifier(t *testing.T) {
	t.Run("identifier with suffix", func(t *testing.T) {
		newStart := lex.Identifier("|", 0)
		if newStart != 0 {
			t.Errorf("non-identifier returned %d instead of %d",
				newStart,
				0)
		}
	})
}

/*
Given a number
When we lex
We should parse the number
*/
func TestInteger(t *testing.T) {
	t.Run("integer", func(t *testing.T) {
		newStart := lex.Integer("123", 0)
		if newStart != 3 {
			t.Errorf("integer returned %d instead of %d",
				newStart,
				3)
		}
	})
}

/*
Given a float
When we lex
We should parse the float
*/
func TestFloat(t *testing.T) {
	t.Run("float", func(t *testing.T) {
		newStart := lex.Float("123.456", 0)
		if newStart != 7 {
			t.Errorf("float returned %d instead of %d",
				newStart,
				7)
		}
	})
}

/*
Given a integer
When we lex a float
We should not parse the integer
*/
func TestFloatWithInteger(t *testing.T) {
	t.Run("float with integer", func(t *testing.T) {
		newStart := lex.Float("123", 0)
		if newStart != 0 {
			t.Errorf("float with integer returned %d instead of %d",
				newStart,
				0)
		}
	})
}

func TestSpecialNumber(t *testing.T) {
	testCases := []struct {
		char byte
		fn   func(string, int) int
	}{
		{'x', lex.Hex},
		{'o', lex.Octal},
		{'b', lex.Binary},
	}

	/*
	   Given a special number
	   When we lex
	   We should parse the special number
	*/
	for _, tc := range testCases {
		t.Run("special number "+string(tc.char), func(t *testing.T) {
			newStart := tc.fn("0"+string(tc.char)+"101", 0)
			if newStart != 5 {
				t.Errorf("special number %s returned %d instead of %d",
					string(tc.char),
					newStart,
					5)
			}

			/*
			   Given non-0-prefixed number
			   When we lex
			   We should not parse
			*/
			newStart = tc.fn("1"+string(tc.char)+"101", 0)
			if newStart != 0 {
				t.Errorf("non-0-prefixed special number %s returned %d instead of %d",
					string(tc.char),
					newStart,
					0)
			}

			/*
			   Given only 0
			   When we lex
			   We should not parse
			*/
			newStart = tc.fn("0", 0)
			if newStart != 0 {
				t.Errorf("only 0 special number %s returned %d instead of %d",
					string(tc.char),
					newStart,
					0)
			}

			/*
			   Given the wrong prefix
			   When we lex
			   We should not parse
			*/
			newStart = tc.fn("0!101", 0)
			if newStart != 0 {
				t.Errorf("wrong prefix special number %s returned %d instead of %d",
					string(tc.char),
					newStart,
					0)
			}
		})
	}

}
