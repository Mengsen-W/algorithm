// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minDeletionSize(strs []string) (ans int) {
	for j := range strs[0] {
		for i := 1; i < len(strs); i++ {
			if strs[i-1][j] > strs[i][j] {
				ans++
				break
			}
		}
	}
	return
}

func main() {
	tests := []struct {
		strs []string
		ans  int
	}{
		{[]string{"cba", "daf", "ghi"}, 1},
		{[]string{"a", "b"}, 0},
		{[]string{"zyx", "wvu", "tsr"}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minDeletionSize(test.strs), "test %d", index)
	}
}
