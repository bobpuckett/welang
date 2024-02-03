package compiler

import (
	"strings"
	"unicode"
)

//go:generate stringer -type=Token
type Token int

const (
	LIST_START Token = iota
	LIST_END
	MAP_START
	MAP_END
	FUNCTION_START
	FUNCTION_END
	TYPE_PARAMETER_START
	TYPE_PARAMETER_END
	TYPE_ALIAS
	TYPE_IDENTITY

	CLAUSE_SEPARATOR
	LIST_SEPARATOR
	IDENTIFIER_SEPARATOR

	DEFINE
	MACRO_SYMBOL
	DISCARD_SYMBOL

	USE_KEYWORD

	INTEGER
	IDENTIFIER
	STRING

	UNKNOWN
)

type Scanner struct {
	Source   string
	Position int

	CurrentToken Token
	CurrentValue string

	Column int
	Row    int
}

// returns false if we've consumed all of source
func (scanner *Scanner) MoveNext() bool {
	if !scanner.hasMore() {
		return false
	}

	nextChar := scanner.Source[scanner.Position]

	for scanner.hasMore() && unicode.IsSpace(rune(nextChar)) {
		scanner.Position += 1
		if scanner.hasMore() {
			nextChar = scanner.Source[scanner.Position]
		}
	}

	if !scanner.hasMore() {
		return false
	}

	return scanner.isSymbol('[', LIST_START) ||
		scanner.isSymbol(']', LIST_END) ||
		scanner.isSymbol('{', MAP_START) ||
		scanner.isSymbol('}', MAP_END) ||
		scanner.isSymbol('{', MAP_START) ||
		scanner.isSymbol('(', FUNCTION_START) ||
		scanner.isSymbol(')', FUNCTION_END) ||
		scanner.isSymbol('<', TYPE_PARAMETER_START) ||
		scanner.isSymbol('>', TYPE_PARAMETER_END) ||
		scanner.isSymbol('\'', TYPE_ALIAS) ||
		scanner.isSymbol('*', TYPE_IDENTITY) ||
		scanner.isSymbol(';', CLAUSE_SEPARATOR) ||
		scanner.isSymbol(',', LIST_SEPARATOR) ||
		scanner.isSymbol(':', DEFINE) ||
		scanner.isSymbol('@', MACRO_SYMBOL) ||
		scanner.isSymbol('_', DISCARD_SYMBOL) ||
		scanner.isSymbol('.', IDENTIFIER_SEPARATOR) ||
		scanner.isKeyword("use", USE_KEYWORD) ||
		scanner.isIdentifier(IDENTIFIER) ||
		scanner.isNumber(INTEGER) ||
		scanner.isString(STRING)
	// TODO: unknown will make an infinite loop
}

func (scanner Scanner) hasMore() bool {
	return scanner.Position < len(scanner.Source)
}

func (scanner *Scanner) isSymbol(char byte, token Token) bool {
	if !scanner.hasMore() || !(scanner.Source[scanner.Position] == char) {
		return false
	}

	scanner.Position += 1
	scanner.CurrentToken = token
	scanner.CurrentValue = string(char)
	return true
}

func (scanner *Scanner) isKeyword(word string, token Token) bool {
	if scanner.Position+len(word) > len(scanner.Source) {
		return false
	}

	for scannerPos, wordPos := scanner.Position, 0; wordPos < len(word); scannerPos, wordPos = scannerPos+1, wordPos+1 {
		if !(scanner.Source[scannerPos] == word[wordPos]) {
			return false
		}
	}

	scanner.Position += len(word)
	scanner.CurrentToken = token
	scanner.CurrentValue = word
	return true
}

func (scanner *Scanner) isIdentifier(token Token) bool {
	current := scanner.Source[scanner.Position]
	if !unicode.IsLetter(rune(current)) {
		return false
	}

	var value strings.Builder
	for unicode.IsLetter(rune(current)) {
		value.WriteByte(current)

		scanner.Position += 1
		if scanner.hasMore() {
			current = scanner.Source[scanner.Position]
		} else {
			break
		}
	}

	scanner.CurrentValue = value.String()
	scanner.CurrentToken = token
	return true
}

func (scanner *Scanner) isNumber(token Token) bool {
	current := scanner.Source[scanner.Position]
	if !unicode.IsNumber(rune(current)) {
		return false
	}

	var value strings.Builder
	for unicode.IsNumber(rune(current)) {
		value.WriteByte(current)

		scanner.Position += 1
		if scanner.hasMore() {
			current = scanner.Source[scanner.Position]
		} else {
			break
		}
	}

	scanner.CurrentValue = value.String()
	scanner.CurrentToken = token
	return true
}

func (scanner *Scanner) isString(token Token) bool {
	current := scanner.Source[scanner.Position]
	if current != '"' {
		return false
	}

	scanner.Position += 1
	if !scanner.hasMore() {
		// TODO: Throw error...
		return false
	}
	current = scanner.Source[scanner.Position]

	// TODO: this is wacky...
	var value strings.Builder
	for current != '"' {
		// Take care of escape sequences
		if current == '\\' {
			value.WriteByte(current)

			scanner.Position += 1
			if scanner.hasMore() {
				current = scanner.Source[scanner.Position]
				println(string(current))
				value.WriteByte(current)

				// TODO: Too many sins
				scanner.Position += 1
				if !scanner.hasMore() {
					break
				}
				current = scanner.Source[scanner.Position]
			}

			// It's possible we have another escape sequence
			continue
		}

		value.WriteByte(current)

		// TODO: wrap this in a function?
		scanner.Position += 1
		if scanner.hasMore() {
			current = scanner.Source[scanner.Position]
		} else {
			break
		}
	}

	scanner.Position += 1
	scanner.CurrentValue = value.String()
	scanner.CurrentToken = token
	return true
}
