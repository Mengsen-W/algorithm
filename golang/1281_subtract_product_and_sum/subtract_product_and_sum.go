/*
 * @Date: 2023-08-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-09
 * @FilePath: /algorithm/golang/1281_subtract_product_and_sum/subtract_product_and_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func subtractProductAndSum(n int) int {
	m := 1
	s := 0
	for n > 0 {
		x := n % 10
		n /= 10
		m *= x
		s += x
	}
	return m - s
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{234, 15},
		{4421, 21},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, subtractProductAndSum(item.n))
	}
}
