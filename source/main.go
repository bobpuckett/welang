package main

import (
	"os"
	"strings"
)

var definitions map[string]Definition

var stringBuilder strings.Builder
var scanner Scanner

func main() {
	if len(os.Args) > 1 {
		Open(os.Args[1])
	} else {
		panic("Please provide a directory or file to parse")
	}
}
