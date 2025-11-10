// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countOperations(num1 int, num2 int) int {
	res := 0 // 相减操作的总次数
	for num1 != 0 && num2 != 0 {
		// 每一步辗转相除操作
		res += num1 / num2
		num1 %= num2
		// 交换两个数
		num1, num2 = num2, num1
	}
	return res
}

func main() {
	tests := []struct {
		num1 int
		num2 int
		ans  int
	}{
		{2, 3, 3},
		{10, 10, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countOperations(test.num1, test.num2), index)
	}
}
