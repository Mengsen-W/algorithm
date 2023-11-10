/*
 * @Date: 2021-11-16 23:38:55
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-10
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxProduct(words []string) (ans int) {
	masks := map[int]int{}
	for _, word := range words {
		mask := 0
		for _, ch := range word {
			mask |= 1 << (ch - 'a')
		}
		if len(word) > masks[mask] {
			masks[mask] = len(word)
		}
	}

	for x, lenX := range masks {
		for y, lenY := range masks {
			if x&y == 0 && lenX*lenY > ans {
				ans = lenX * lenY
			}
		}
	}
	return
}

func main() {
	tests := []struct {
		words []string
		ans   int
	}{
		{[]string{"abcw", "baz", "foo", "bar", "xtfn", "abcdef"}, 16},
		{[]string{"a", "ab", "abc", "d", "cd", "bcd", "abcd"}, 4},
		{[]string{"a", "aa", "aaa", "aaaa"}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxProduct(test.words), index)
	}
}
