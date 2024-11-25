// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func smallestRange(nums [][]int) []int {
	min := func(x, y int) int {
		if x < y {
			return x
		}
		return y
	}

	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}

	size := len(nums)
	indices := map[int][]int{}
	xMin, xMax := math.MaxInt32, math.MinInt32
	for i := 0; i < size; i++ {
		for _, x := range nums[i] {
			indices[x] = append(indices[x], i)
			xMin = min(xMin, x)
			xMax = max(xMax, x)
		}
	}
	freq := make([]int, size)
	inside := 0
	left, right := xMin, xMin-1
	bestLeft, bestRight := xMin, xMax
	for right < xMax {
		right++
		if len(indices[right]) > 0 {
			for _, x := range indices[right] {
				freq[x]++
				if freq[x] == 1 {
					inside++
				}
			}
			for inside == size {
				if right-left < bestRight-bestLeft {
					bestLeft, bestRight = left, right
				}
				for _, x := range indices[left] {
					freq[x]--
					if freq[x] == 0 {
						inside--
					}
				}
				left++
			}
		}
	}
	return []int{bestLeft, bestRight}
}

func main() {
	tests := []struct {
		nums [][]int
		ans  []int
	}{
		{[][]int{{4, 10, 15, 24, 26}, {0, 9, 12, 20}, {5, 18, 22, 30}}, []int{20, 24}},
		{[][]int{{1, 2, 3}, {1, 2, 3}, {1, 2, 3}}, []int{1, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, smallestRange(test.nums), index)
	}
}
