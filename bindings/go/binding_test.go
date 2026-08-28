package tree_sitter_alg_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_alg "none/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_alg.Language())
	if language == nil {
		t.Errorf("Error loading Alg grammar")
	}
}
