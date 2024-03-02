package main

import "unicode"

type Scanner interface {
	Current() rune
	Move() rune
	Peek() rune
	PeekTwo() rune
	Recale() rune // looks back
	SkipWhitespace() rune
}

type StringScanner struct {
	source   []byte
	position int

	Row    int
	Column int
}

func (s *StringScanner) Current() rune {
	if s.position >= len(s.source) {
		return -1
	}

	return rune(s.source[s.position])
}

func (s *StringScanner) Move() rune {
	s.position += 1

	if s.position >= len(s.source) {
		return -1
	}

	next := rune(s.source[s.position])

	incrementRowColumn(next, s)

	return next
}
func (s *StringScanner) Peek() rune {
	if s.position+1 >= len(s.source) {
		return -1
	}
	return rune(s.source[s.position+1])
}
func (s *StringScanner) PeekTwo() rune {
	if s.position+2 >= len(s.source) {
		return -1
	}
	return rune(s.source[s.position+2])
}
func (s *StringScanner) Recale() rune {
	if s.position < 1 {
		return -1
	}
	return rune(s.source[s.position-1])
}
func (s *StringScanner) SkipWhitespace() rune {
	for !(s.Current() == -1) && unicode.IsSpace(s.Current()) {
		s.Move()
	}
	return s.Current()
}

func incrementRowColumn(next rune, s *StringScanner) {
	if next == '\n' {
		s.Row = 0
		s.Column += 1
	} else {
		s.Row += 1
	}
}
