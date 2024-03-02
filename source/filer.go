package main

import (
	"os"
	"path/filepath"
	"strings"
)

func Open(path string) Definition {
	stats, err := os.Stat(path)

	if err != nil {
		panic(err)
	}

	if stats.IsDir() {
		entries, err := os.ReadDir(path)
		if err != nil {
			panic(err)
		}

		defs := make(map[string]Definition)

		for _, entry := range entries {
			filename := strings.TrimSuffix(entry.Name(), filepath.Ext(entry.Name()))
			defs[filename] = Open(path + string('/') + entry.Name())
		}
		return Map{
			Definitions: defs,
		}
	} else if filepath.Ext(path) == ".we" {
		content, err := os.ReadFile(path)
		if err != nil {
			panic(err)
		}

		scanner = &StringScanner{
			source:   content,
			position: 0,
		}

		return ParseModule()
	}
}
