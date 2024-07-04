package main

import (
	"slices"
	"strconv"
	"unicode"
)

func ParseModule() Definition {
	defs := make(map[string]Definition)
	for scanner.Current() != -1 {
		next := Parse()
		if next == nil {
			break
		}

		key := ParseReference().String()
		if scanner.Current() != ':' {
			panic("missing : after " + key)
		}
		value := Parse()

		defs[key] = value
	}

	return Map{
		Definitions: defs,
	}
}

func Parse() Definition {
	nextChar := scanner.SkipWhitespace()

	if nextChar == -1 {
		return nil
	}

	switch {
	case nextChar == '(':
		return ParseWord()
	case nextChar == '"':
		return ParseString()
	case nextChar == '`':
		return ParseInvoke()
	case nextChar == '$':
		return ParseEnvironmentVariable()
	case nextChar == '{':
		return ParseMap()
	case unicode.IsLetter(nextChar):
		return ParseReference()
	case unicode.IsDigit(nextChar):
		return ParseNumber()
	default:
		panic("encountered unknown starting symbol " + string(nextChar))
	}
}

func ParseWord() Definition {
	scanner.Move()

	steps := make([]Definition, 0)
	chain := make([]Definition, 0)

	for scanner.Current() != -1 {

		if scanner.Current() == ')' {
			steps = append(steps, chain...)
			chain = nil // Apparently this is the idiomatic way to clear the slice?
			scanner.Move()
			break
		}

		if scanner.Current() == '|' {
			steps = append(steps, chain...)
			chain = nil // Apparently this is the idiomatic way to clear the slice?
			scanner.Move()
			continue
		}

		parsed := Parse()
		if parsed != nil { // Happens if there's just whitespace left
			chain = append(chain, parsed)
		}

	}

	return Word{
		Definitions: slices.Reverse(steps),
	}
}

func ParseString() Definition {
	scanner.Move() // move past "

	for scanner.Current() != -1 && (scanner.Current() != '"' || scanner.Recale() == '\\') {
		stringBuilder.WriteRune(scanner.Current())
		scanner.Move()
	}
	scanner.Move() // Move past last "

	result := String{
		value: stringBuilder.String(),
	}
	stringBuilder.Reset()

	return result
}

func ParseInvoke() Definition {
	scanner.Move()

	// TODO: format should be a split of strings and the arguments associated
	parts := make([]string, 0)
	for scanner.Current() != -1 && (scanner.Current() != '`' || scanner.Recale() == '\\') {
		if scanner.Current() == '+' {
			parts = append(parts, stringBuilder.String())
			stringBuilder.Reset()

			stringBuilder.WriteRune(scanner.Current())

			scanner.Move()
			for unicode.IsNumber(scanner.Current()) {
				stringBuilder.WriteRune(scanner.Current())
				scanner.Move()
			}

			for scanner.Current() == '.' {
				reference := ParseReference()
				stringBuilder.WriteString(reference.String())
			}

			parts = append(parts, stringBuilder.String())
			stringBuilder.Reset()

			continue
		} else {
			stringBuilder.WriteRune(scanner.Current())
			scanner.Move()
		}
	}

	parts = append(parts, stringBuilder.String())
	stringBuilder.Reset()

	scanner.Move() // Move past the last `

	result := Script{
		Parts: parts,
	}

	stringBuilder.Reset()

	return result
}

func ParseEnvironmentVariable() Definition {
	scanner.Move()

	for unicode.IsLetter(scanner.Current()) {
		stringBuilder.WriteRune(scanner.Current())
		scanner.Move()
	}

	result := EnvironmentVariable{
		Name: stringBuilder.String(),
	}
	stringBuilder.Reset()

	return result
}

func ParseReference() Definition {
	for unicode.IsLetter(scanner.Current()) {
		stringBuilder.WriteRune(scanner.Current())
		scanner.Move()
	}

	result := Reference{
		Name: stringBuilder.String(),
	}
	stringBuilder.Reset()

	return result
}

func ParseMap() Definition {
	scanner.Move()

	definitions := make(map[string][]Definition)
	for scanner.Current() != -1 && scanner.Current() != '}' {
		scanner.SkipWhitespace()

		key := ParseReference()

		if scanner.SkipWhitespace() != ':' {
			panic("No define symbol after map")
		}
		scanner.Move()

		values := make([]Definition, 0)
		for scanner.SkipWhitespace() != -1 && scanner.Current() != ',' && scanner.Current() != '}' {
			values = append(values, Parse())
		}

		definitions[key.String()] = values
	}
	scanner.Move()

	result := Map{
		Definitions: definitions,
	}

	return result
}

func ParseNumber() Definition {
	for unicode.IsDigit(scanner.Current()) {
		stringBuilder.WriteRune(scanner.Current())
		scanner.Move()
	}

	value, err := strconv.Atoi(stringBuilder.String())
	if err != nil {
		panic(err.Error())
	}
	stringBuilder.Reset()

	return Number{
		Value: value,
	}
}

// type ParseError struct {
// 	message string
// 	column  int
// 	row     int
// }
