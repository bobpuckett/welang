package cmd

type Node struct {
}

func (n *Node) ParseModule() {}

func (n *Node) parseFunction() {}

func (n *Node) parseFunctionType() {}

func (n *Node) parseAlias() {}

func (n *Node) parseIdentity() {}

func (n *Node) parseAtom() {}

func (n *Node) parseUnsigned() {}

func (n *Node) parseSigned() {}

func (n *Node) parseReal() {}

func (n *Node) parseAtomType() {}

func (n *Node) parseContext() {}

func (n *Node) parseContextType() {}

func (n *Node) parseMap() {}

func (n *Node) parseMapType() {}

func (n *Node) parseArray() {}

func (n *Node) parseArrayType() {}

func (n *Node) parseTuple() {}

func (n *Node) parseTupleType() {}

func (n *Node) parseUse() {}
