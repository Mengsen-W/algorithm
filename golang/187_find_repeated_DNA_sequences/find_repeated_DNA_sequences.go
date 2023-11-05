/*
 * @Date: 2021-10-08 00:13:26
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-05
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const L = 10

var bin = map[byte]int{'A': 0, 'C': 1, 'G': 2, 'T': 3}

func findRepeatedDnaSequences(s string) (ans []string) {
	n := len(s)
	if n <= L {
		return
	}
	x := 0
	for _, ch := range s[:L-1] {
		x = x<<2 | bin[byte(ch)]
	}
	cnt := map[int]int{}
	for i := 0; i <= n-L; i++ {
		x = (x<<2 | bin[s[i+L-1]]) & (1<<(L*2) - 1)
		cnt[x]++
		if cnt[x] == 2 {
			ans = append(ans, s[i:i+L])
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s   string
		ans []string
	}{
		{"AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT", []string{"AAAAACCCCC", "CCCCCAAAAA"}},
		{"AAAAAAAAAAAAA", []string{"AAAAAAAAAA"}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findRepeatedDnaSequences(test.s), index)
	}
}
