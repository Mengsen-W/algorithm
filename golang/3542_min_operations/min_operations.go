// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperations(nums []int) int {
	s := []int{}
	res := 0
	for _, a := range nums {
		for len(s) > 0 && s[len(s)-1] > a {
			s = s[:len(s)-1]
		}
		if a == 0 {
			continue
		}
		if len(s) == 0 || s[len(s)-1] < a {
			res++
			s = append(s, a)
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{0, 2}, 1},
		{[]int{3, 1, 2, 1}, 3},
		{[]int{1, 2, 1, 2, 1, 2}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, minOperations(test.nums), test.ans, index)
	}

}
