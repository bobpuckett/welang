package compiler

import (
	"testing"
)

func TestHandlesWhitespace(t *testing.T) {
	scanner := Scanner{}
	scanner.Source = " hi : ( bob.the.palindrome ) 123, "
	assertNext(t, &scanner, IDENTIFIER, "hi")
	assertNext(t, &scanner, DEFINE, ":")
	assertNext(t, &scanner, FUNCTION_START, "(")
	assertNext(t, &scanner, IDENTIFIER, "bob")
	assertNext(t, &scanner, IDENTIFIER_SEPARATOR, ".")
	assertNext(t, &scanner, IDENTIFIER, "the")
	assertNext(t, &scanner, IDENTIFIER_SEPARATOR, ".")
	assertNext(t, &scanner, IDENTIFIER, "palindrome")
	assertNext(t, &scanner, FUNCTION_END, ")")
	assertNext(t, &scanner, INTEGER, "123")
	assertNext(t, &scanner, LIST_SEPARATOR, ",")
}

func TestCanEndWithIdentifierChain(t *testing.T) {
	scanner := Scanner{}
	scanner.Source = "hello.from.the.out.side"
	assertNext(t, &scanner, IDENTIFIER, "hello")
	assertNext(t, &scanner, IDENTIFIER_SEPARATOR, ".")
	assertNext(t, &scanner, IDENTIFIER, "from")
	assertNext(t, &scanner, IDENTIFIER_SEPARATOR, ".")
	assertNext(t, &scanner, IDENTIFIER, "the")
	assertNext(t, &scanner, IDENTIFIER_SEPARATOR, ".")
	assertNext(t, &scanner, IDENTIFIER, "out")
	assertNext(t, &scanner, IDENTIFIER_SEPARATOR, ".")
	assertNext(t, &scanner, IDENTIFIER, "side")
}

func TestCanLexString(t *testing.T) {
	scanner := Scanner{}
	scanner.Source = "anyString: \"any\\\"String\"\nanyInt: 100\n"
	println(scanner.Source)
	assertNext(t, &scanner, IDENTIFIER, "anyString")
	assertNext(t, &scanner, DEFINE, ":")
	assertNext(t, &scanner, STRING, "any\\\"String")
	assertNext(t, &scanner, IDENTIFIER, "anyInt")
	assertNext(t, &scanner, DEFINE, ":")
	assertNext(t, &scanner, INTEGER, "100")
}

func assertNext(t *testing.T, scanner *Scanner, token Token, value string) {
	success := scanner.MoveNext()

	if !success {
		t.Errorf("Lexer was not able to find a matching token at position %d", scanner.Position)
	}
	if scanner.CurrentToken != token ||
		scanner.CurrentValue != value {

		t.Errorf("Failed to lex (%s|%s) (%s|%s)", token, value, scanner.CurrentToken, scanner.CurrentValue)
	}

	if !t.Failed() {
		t.Logf("Lexed: %s|%s", token.String(), value)
	}
}
