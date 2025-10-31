// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func getSneakyNumbers(nums []int) []int {
	n := len(nums) - 2
	sum, squaredSum := 0.0, 0.0
	for _, x := range nums {
		sum += float64(x)
		squaredSum += float64(x * x)
	}
	sum2 := sum - float64(n*(n-1)/2)
	squaredSum2 := squaredSum - float64(n*(n-1)*(2*n-1)/6)
	x1 := (sum2 - math.Sqrt(2*squaredSum2-sum2*sum2)) / 2
	x2 := (sum2 + math.Sqrt(2*squaredSum2-sum2*sum2)) / 2
	return []int{int(x1), int(x2)}
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{0, 1, 1, 0}, []int{0, 1}},
		{[]int{0, 3, 2, 1, 3, 2}, []int{2, 3}},
		{[]int{7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2}, []int{4, 5}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getSneakyNumbers(test.nums), index)
	}
}
