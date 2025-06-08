// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func lexicalOrder(n int) []int {
	ans := make([]int, n)
	num := 1
	for i := range ans {
		ans[i] = num
		if num*10 <= n {
			num *= 10
		} else {
			for num%10 == 9 || num+1 > n {
				num /= 10
			}
			num++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		n   int
		ans []int
	}{
		{13, []int{1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9}},
		{2, []int{1, 2}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, lexicalOrder(test.n), index)
	}
}
