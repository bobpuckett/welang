package parse

type Node struct {
	SelfType Type
	InType   Type
	OutType  Type
	Start    uint
	End      uint
}

type Type struct {
	Claim        string
	Requirements map[string]Type
}
