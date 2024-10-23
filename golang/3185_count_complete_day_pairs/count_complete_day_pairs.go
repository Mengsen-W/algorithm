// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countCompleteDayPairs(hours []int) int64 {
	var ans int64 = 0
	cnt := make([]int, 24)
	for _, hour := range hours {
		ans += int64(cnt[(24-hour%24)%24])
		cnt[hour%24]++
	}
	return ans
}

func main() {
	tests := []struct {
		hours []int
		ans   int64
	}{
		{[]int{12, 12, 30, 24, 24}, 2},
		{[]int{72, 48, 24, 3}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countCompleteDayPairs(test.hours), index)
	}
}
