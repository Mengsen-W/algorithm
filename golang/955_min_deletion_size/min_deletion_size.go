// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minDeletionSize(strs []string) int {
	// cuts[i] is True : we don't need to check col[i] <= col[i+1]
	cuts := make([]bool, len(strs)-1)
	ans := 0
	for j := 0; j < len(strs[0]); j++ {
		canKeep := true
		for i := 0; i < len(strs)-1; i++ {
			if !cuts[i] && strs[i][j] > strs[i+1][j] {
				canKeep = false
				break
			}
		}
		if canKeep {
			for i := 0; i < len(strs)-1; i++ {
				if strs[i][j] < strs[i+1][j] {
					cuts[i] = true
				}
			}
		} else {
			ans++
		}
	}

	return ans
}

func main() {
	tests := []struct {
		strs []string
		ans  int
	}{
		{[]string{"ca", "bb", "ac"}, 1},
		{[]string{"xc", "yb", "za"}, 0},
		{[]string{"zyx", "wvu", "tsr"}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minDeletionSize(test.strs), index)
	}
}
