package lex

import "fmt"

func UseKeyword(source string, start int) int {
	return keyword(source, start, "use")
}

func AliasKeyword(source string, start int) int {
	return keyword(source, start, "alias")
}

func IdentityKeyword(source string, start int) int {
	return keyword(source, start, "identity")
}

func FunctionKeyword(source string, start int) int {
	return keyword(source, start, "fn")
}

func keyword(source string, start int, keyword string) int {
	if start > 0 && isIdentifierlike(source[start-1]) {
		// TODO: this means we have a bug in the compiler.
		err := fmt.Errorf("possible keyword prefixed by identifier: %s", string(source[start-1]))
		if err != nil {
			println(err.Error())
		}

		return start
	}

	for i, j := start, 0; j < len(keyword); i, j = i+1, j+1 {
		if source[i] != keyword[j] {
			return start
		}
	}

	newStart := start + len(keyword)

	if len(source) <= newStart {
		return start + len(keyword)
	}

	nextChar := source[newStart]
	if between(48, nextChar, 57) { // 0-9
		return start
	} else if between(65, nextChar, 90) { // A-Z
		return start
	} else if between(97, nextChar, 122) { // a-z
		return start
	} else if nextChar == 95 { // _
		return start
	}

	return newStart
}

func isIdentifierlike(char byte) bool {
	return isNumeric(char) || // 0-9
		isAlphabetic(char) || // A-Z, a-z
		char == 95 // _
}

func isNumeric(char byte) bool {
	return between(48, char, 57) // 0-9
}

func isAlphabetic(char byte) bool {
	return between(65, char, 90) || // A-Z
		between(97, char, 122) // a-z
}

func between(lower byte, char byte, upper byte) bool {
	return lower <= char && char <= upper
}
