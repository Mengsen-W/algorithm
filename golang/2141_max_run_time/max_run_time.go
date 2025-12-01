// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxRunTime(n int, batteries []int) int64 {
	sum := int64(0)
	for _, cap := range batteries {
		sum += int64(cap)
	}

	left, right := int64(0), sum/int64(n)
	ans := int64(0)
	for left <= right {
		mid := left + (right-left)/2
		total := int64(0)
		for _, cap := range batteries {
			if int64(cap) < mid {
				total += int64(cap)
			} else {
				total += mid
			}
		}
		if total >= int64(n)*mid {
			ans = mid
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	return ans
}

func main() {
	tests := []struct {
		n         int
		batteries []int
		ans       int64
	}{
		{2, []int{3, 3, 3}, 4},
		{2, []int{1, 1, 1, 1}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxRunTime(test.n, test.batteries), index)
	}
}
