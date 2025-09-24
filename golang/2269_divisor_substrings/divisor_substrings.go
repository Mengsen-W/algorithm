// Package main ...
package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func divisorSubstrings(num int, k int) int {
	s := strconv.Itoa(num) // num 十进制表示字符串
	n := len(s)
	res := 0
	for i := 0; i <= n-k; i++ {
		// 枚举所有长度为 k 的子串
		tmp, _ := strconv.Atoi(s[i : i+k])
		if tmp != 0 && num%tmp == 0 {
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		num int
		k   int
		ans int
	}{
		{240, 2, 2},
		{430043, 2, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, divisorSubstrings(test.num, test.k), index)
	}
}
