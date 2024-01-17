/*
 * @Date: 2024-01-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-17
 * @FilePath: /algorithm/golang/2744_maximum_number_of_string_pairs/maximum_number_of_string_pairs.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumNumberOfStringPairs(words []string) int {
	ans := 0
	seen := map[int]int{}
	for _, word := range words {
		ans += seen[int(word[1])*100+int(word[0])]
		seen[int(word[0])*100+int(word[1])] = 1
	}
	return ans
}

func main() {
	tests := []struct {
		words []string
		ans   int
	}{
		{[]string{"cd", "ac", "dc", "ca", "zz"}, 2},
		{[]string{"ab", "ba", "cc"}, 1},
		{[]string{"aa", "ab"}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumNumberOfStringPairs(test.words), index)
	}
}
