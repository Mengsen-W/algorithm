// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func partition(s string) (ans [][]string) {
	n := len(s)
	f := make([][]bool, n)
	for i := range f {
		f[i] = make([]bool, n)
		for j := range f[i] {
			f[i][j] = true
		}
	}
	for i := n - 1; i >= 0; i-- {
		for j := i + 1; j < n; j++ {
			f[i][j] = s[i] == s[j] && f[i+1][j-1]
		}
	}

	splits := []string{}
	var dfs func(int)
	dfs = func(i int) {
		if i == n {
			ans = append(ans, append([]string(nil), splits...))
			return
		}
		for j := i; j < n; j++ {
			if f[i][j] {
				splits = append(splits, s[i:j+1])
				dfs(j + 1)
				splits = splits[:len(splits)-1]
			}
		}
	}
	dfs(0)
	return
}

func main() {
	tests := []struct {
		s   string
		ans [][]string
	}{
		{"aab", [][]string{{"a", "a", "b"}, {"aa", "b"}}},
		{"a", [][]string{{"a"}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, partition(test.s), index)
	}
}
