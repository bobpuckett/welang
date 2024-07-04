package cmd

import (
	"github.com/winstonpuckett/welang/src/internal/lex"
)

func Main() {
	_ = lex.Identifier("|anyIdentifier", 1)
}
