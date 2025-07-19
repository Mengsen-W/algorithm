// Package main ...
package main

import (
	"sort"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func removeSubfolders(folder []string) (ans []string) {
	sort.Strings(folder)
	ans = append(ans, folder[0])
	for _, f := range folder[1:] {
		last := ans[len(ans)-1]
		if !strings.HasPrefix(f, last) || f[len(last)] != '/' {
			ans = append(ans, f)
		}
	}
	return
}

func main() {
	tests := []struct {
		folder []string
		ans    []string
	}{
		{[]string{"/a", "/a/b", "/c/d", "/c/d/e", "/c/f"}, []string{"/a", "/c/d", "/c/f"}},
		{[]string{"/a", "/a/b/c", "/a/b/d"}, []string{"/a"}},
		{[]string{"/a/b/c", "/a/b/ca", "/a/b/d"}, []string{"/a/b/c", "/a/b/ca", "/a/b/d"}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, removeSubfolders(test.folder), index)
	}
}
