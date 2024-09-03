package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxStrength(nums []int) int64 {
	negativeCount, zeroCount, positiveCount := 0, 0, 0
	prod := 1
	maxNegative := math.MinInt32

	for _, num := range nums {
		if num < 0 {
			negativeCount++
			prod *= num
			if num > maxNegative {
				maxNegative = num
			}
		} else if num == 0 {
			zeroCount++
		} else {
			prod *= num
			positiveCount++
		}
	}

	if negativeCount == 1 && zeroCount == 0 && positiveCount == 0 {
		return int64(nums[0])
	}
	if negativeCount <= 1 && positiveCount == 0 {
		return int64(0)
	}
	if prod < 0 {
		return int64(prod / maxNegative)
	}
	return int64(prod)
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{3, -1, -5, 2, 5, -9}, 1350},
		{[]int{-4, -5, -4}, 20},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxStrength(test.nums), index)
	}
}
