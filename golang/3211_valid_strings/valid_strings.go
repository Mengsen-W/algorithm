// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func validStrings(n int) []string {
	res := []string{}
	var dfs func(arr []rune)
	dfs = func(arr []rune) {
		if len(arr) == n {
			res = append(res, string(arr))
			return
		}
		if len(arr) == 0 || arr[len(arr)-1] == '1' {
			arr = append(arr, '0')
			dfs(arr)
			arr = arr[:len(arr)-1]
		}
		arr = append(arr, '1')
		dfs(arr)
		arr = arr[:len(arr)-1]
	}
	dfs([]rune{})
	return res
}

func main() {
	tests := []struct {
		n   int
		ans []string
	}{
		{3, []string{"010", "011", "101", "110", "111"}},
		{1, []string{"0", "1"}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, validStrings(test.n), index)
	}
}
