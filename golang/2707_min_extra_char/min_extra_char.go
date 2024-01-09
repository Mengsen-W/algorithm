/*
 * @Date: 2024-01-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-09
 * @FilePath: /algorithm/golang/2707_min_extra_char/min_extra_char.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minExtraChar(s string, dictionary []string) int {
	n := len(s)
	d := make([]int, n+1)
	for i := 1; i <= n; i++ {
		d[i] = math.MaxInt
	}
	mp := map[string]int{}
	for _, e := range dictionary {
		mp[e]++
	}
	for i := 1; i <= n; i++ {
		d[i] = d[i-1] + 1
		for j := i - 1; j >= 0; j-- {
			if _, ok := mp[s[j:i]]; ok {
				d[i] = min(d[i], d[j])
			}
		}
	}
	return d[n]
}

func main() {
	tests := []struct {
		s          string
		dictionary []string
		ans        int
	}{
		{"leetscode", []string{"leet", "code", "leetcode"}, 1},
		{"sayhelloworld", []string{"hello", "world"}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minExtraChar(test.s, test.dictionary), index)
	}
}
