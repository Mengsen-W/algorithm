// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canAliceWin(nums []int) bool {
	singleDigitSum := 0
	doubleDigitSum := 0

	for _, num := range nums {
		if num < 10 {
			singleDigitSum += num
		} else {
			doubleDigitSum += num
		}
	}

	return singleDigitSum != doubleDigitSum
}

func main() {
	tests := []struct {
		nums []int
		ans  bool
	}{
		{[]int{1, 2, 3, 4, 10}, false},
		{[]int{1, 2, 3, 4, 5, 14}, true},
		{[]int{5, 5, 5, 25}, true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canAliceWin(test.nums), index)
	}
}
