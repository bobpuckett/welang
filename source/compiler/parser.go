package compiler

import "fmt"

type TypeId int

const (
	UNVISITED TypeId = iota
	IDENTITY
	ALIAS
	CONTEXT
	ARRAY
	ATOM
	NONE
)

type Type struct {
	TypeId    TypeId
	Reference string // optional
	SubType   *Type  // optional
}

type TypePair struct {
	In  Type
	Out Type
}

type ValueTypeId int

const (
	Module ValueTypeId = iota
	Array
	MAP
	FUNCTION
	TYPE_ALIAS
	TYPE_IDENTITY
	DISCARD
	INTEGER
	IDENTIFIER_CHAIN
	STRING
)

type IdentifierChain []string
type Value struct {
	ValueType ValueTypeId

	Usings IdentifierChain // optional
	Map    map[string]*Value
	List   []*Node
	Sub    *Node
	Atom   int
}

type Node struct {
	InferredType      TypePair // is null before the type step
	ParameterizedType TypePair // optional

	Value Value
}

type Parser struct {
	Scanner *Scanner
}

func (parser *Parser) MoveNext() bool {
	switch parser.Scanner.CurrentToken {
	case LIST_START:
		parser.ParseList()
	case LIST_END:
		panic("MoveNext encountered end of list")
	case MAP_START:
		parser.ParseMap()
	case MAP_END:
		panic("MoveNext encountered end of map")
	case FUNCTION_START:
		parser.ParseFunction()
	case FUNCTION_END:
		panic("MoveNext encountered end of function")
	case TYPE_PARAMETER_START:
		parser.ParseTypeParameterAndValue()
	case TYPE_PARAMETER_END:
		panic("MoveNext encountered end of type parameter")
	case TYPE_ALIAS_SYMBOL:
		parser.ParseTypeAlias()
	case TYPE_IDENTITY_SYMBOL:
		parser.ParseTypeIdentity()
	case CLAUSE_SEPARATOR:
		panic("MoveNext encountered clause separator")
	case LIST_SEPARATOR:
		panic("MoveNext encountered list separator")
	case IDENTIFIER_SEPARATOR:
		panic("MoveNext encountered identifier separator")
	case DEFINE:
		panic("MoveNext encountered define symbol")
	case MACRO_SYMBOL:
		parser.ParseMacro()
	case DISCARD_SYMBOL:
		parser.ParseDiscard()
	case USE_KEYWORD:
		parser.ParseUseStatement()
	case INTEGER_LITERAL:
		parser.ParseAtom()
	case IDENTIFIER:
		parser.ParseIdentifierChain()
	case STRING_LITERAL:
		parser.ParseString()
	case UNKNOWN:
		panic("MoveNext encountered end of list")
	}

	return true
}

func (parser *Parser) hasMore() bool {
	return parser.Scanner.hasMore()
}

func (parser *Parser) ParseList() {
	if parser.Scanner.CurrentToken != LIST_START {
		panic(fmt.Sprintf("Tried to parse %s as list", LIST_START.String()))
	}
	// Note this advances the scanner
	if !parser.Scanner.MoveNext() {
		panic("Reached end of input before finding a closing ] bracket")
	}

	foundSeparator := true;
	for {
		switch
	}
}

func (parser *Parser) ParseMap() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseFunction() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseTypeParameterAndValue() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseTypeAlias() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseTypeIdentity() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseMacro() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseDiscard() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseUseStatement() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseAtom() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseIdentifierChain() {
	panic("Not Implemented...")
}

func (parser *Parser) ParseString() {
	panic("Not Implemented...")
}
