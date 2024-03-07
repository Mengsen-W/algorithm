/*
 * @Date: 2024-03-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-07
 * @FilePath: /algorithm/golang/2575_divisibility_array/divisibility_array.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func divisibilityArray(word string, m int) []int {
	res := make([]int, 0)
	cur := 0
	for _, c := range word {
		cur = (cur*10 + int(c-'0')) % m
		if cur == 0 {
			res = append(res, 1)
		} else {
			res = append(res, 0)
		}
	}
	return res
}

func main() {
	tests := []struct {
		word string
		m    int
		want []int
	}{
		{"998244353", 3, []int{1, 1, 0, 0, 0, 1, 1, 0, 0}},
		{"1010", 10, []int{0, 1, 0, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.want, divisibilityArray(test.word, test.m), index)
	}
}
