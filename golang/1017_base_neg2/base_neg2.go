/*
 * @Date: 2023-04-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-28
 * @FilePath: /algorithm/golang/1017_base_neg2/base_neg2.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func baseNeg2(n int) string {
	val := 0x55555555 ^ (0x55555555 - n)
	if val == 0 {
		return "0"
	}
	res := []byte{}
	for val > 0 {
		res = append(res, '0'+byte(val&1))
		val >>= 1
	}
	for i, n := 0, len(res); i < n/2; i++ {
		res[i], res[n-1-i] = res[n-1-i], res[i]
	}
	return string(res)
}

func main() {
	tests := []struct {
		n   int
		ans string
	}{
		{2, "110"},
		{3, "111"},
		{4, "100"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, baseNeg2(test.n), index)
	}
}
