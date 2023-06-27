/*
 * @Date: 2023-06-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-15
 * @FilePath: /algorithm/golang/1777_can_make_pali_queries/can_make_pali_queries.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canMakePaliQueries(s string, queries [][]int) []bool {
	n := len(s)
	count := make([]int, n+1)
	for i := 0; i < n; i++ {
		count[i+1] = count[i] ^ (1 << (s[i] - 'a'))
	}
	res := make([]bool, len(queries))
	for i, query := range queries {
		l := query[0]
		r := query[1]
		k := query[2]
		bits := 0
		x := count[r+1] ^ count[l]
		for x > 0 {
			x &= x - 1
			bits++
		}
		res[i] = bits <= k*2+1
	}
	return res
}

func main() {
	s := "abcda"
	queries := [][]int{{3, 3, 0}, {1, 2, 0}, {0, 3, 1}, {0, 3, 2}, {0, 4, 1}}
	ans := []bool{true, false, false, true, true}
	assert.Equal(&testing.T{}, canMakePaliQueries(s, queries), ans)
}
