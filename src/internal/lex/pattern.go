package lex

func Identifier(source string, start int) int {
	if !isAlphabetic(source[start]) {
		return start
	}

	for start < len(source) && isIdentifierlike(source[start]) {
		start += 1
	}

	return start
}

func Integer(source string, start int) int {
	for start < len(source) && isNumeric(source[start]) {
		start += 1
	}

	return start
}

func Float(source string, start int) int {
	next := Integer(source, start)
	if next == start || next >= len(source) {
		return start
	}

	if source[next] == '.' {
		next += 1
		next = Integer(source, next)
	}

	return next
}

func Hex(source string, start int) int {
	return specialNumber(source, start, 'x')
}

func Octal(source string, start int) int {
	return specialNumber(source, start, 'o')
}

func Binary(source string, start int) int {
	return specialNumber(source, start, 'b')
}

func specialNumber(source string, start int, prefix byte) int {
	if source[start] != '0' {
		return start
	}

	if start+1 >= len(source) || source[start+1] != prefix {
		return start
	}

	start += 2

	for start < len(source) && isNumeric(source[start]) {
		start += 1
	}

	return start
}
