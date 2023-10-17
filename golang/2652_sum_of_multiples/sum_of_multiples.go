/*
 * @Date: 2023-10-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-17
 * @FilePath: /algorithm/golang/2652_sum_of_multiples/sum_of_multiples.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumOfMultiples(n int) int {
	f := func(n, m int) int {
		return (m + n/m*m) * (n / m) / 2
	}
	return f(n, 3) + f(n, 5) + f(n, 7) - f(n, 3*5) - f(n, 3*7) - f(n, 5*7) + f(n, 3*5*7)
}

func main() {
	tests := []struct {
		n    int
		want int
	}{
		{7, 21},
		{10, 40},
		{9, 30},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.want, sumOfMultiples(test.n), "case(%d) failed", index)
	}
}
