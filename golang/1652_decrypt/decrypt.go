/*
 * @Date: 2022-09-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-05
 * @FilePath: /algorithm/golang/1652_decrypt/decrypt.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func decrypt(code []int, k int) []int {
	n := len(code)
	ans := make([]int, n)
	if k == 0 {
		return ans
	}
	code = append(code, code...)
	l, r := 1, k
	if k < 0 {
		l, r = n+k, n-1
	}
	sum := 0
	for _, v := range code[l : r+1] {
		sum += v
	}
	for i := range ans {
		ans[i] = sum
		sum -= code[l]
		sum += code[r+1]
		l, r = l+1, r+1
	}
	return ans
}

func main() {
	tests := []struct {
		code []int
		k    int
		ans  []int
	}{
		{[]int{5, 7, 1, 4}, 3, []int{12, 10, 16, 13}},
		{[]int{1, 2, 3, 4}, 0, []int{0, 0, 0, 0}},
		{[]int{2, 4, 9, 3}, -2, []int{12, 5, 6, 13}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, decrypt(test.code, test.k), index)
	}
}
