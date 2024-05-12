package tree_sitter_mojo_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/lsh/tree-sitter-mojo"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_mojo.Language())
	if language == nil {
		t.Errorf("Error loading Python grammar")
	}
}
