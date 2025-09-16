// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func replaceNonCoprimes(nums []int) []int {
	ans := []int{}
	for _, num := range nums {
		for len(ans) > 0 {
			last := ans[len(ans)-1]
			g := gcd(last, num)
			if g > 1 {
				num = last / g * num
				ans = ans[:len(ans)-1]
			} else {
				break
			}
		}
		ans = append(ans, num)
	}
	return ans
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{6, 4, 3, 2, 7, 6, 2}, []int{12, 7, 6}},
		{[]int{2, 2, 1, 1, 3, 3, 3}, []int{2, 1, 1, 3}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, replaceNonCoprimes(test.nums), index)
	}
}
