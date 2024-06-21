// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func temperatureTrend(temperatureA []int, temperatureB []int) int {
	n := len(temperatureA)
	ans := 0
	cur := 0
	for i := 1; i < n; i++ {
		ta := getTrend(temperatureA[i-1], temperatureA[i])
		tb := getTrend(temperatureB[i-1], temperatureB[i])
		if ta == tb {
			cur++
			ans = max(ans, cur)
		} else {
			cur = 0
		}
	}
	return ans
}

func getTrend(x, y int) int {
	if x == y {
		return 0
	}
	if x < y {
		return -1
	}
	return 1
}

func main() {
	tests := []struct {
		temperatureA []int
		temperatureB []int
		ans          int
	}{
		{[]int{21, 18, 18, 18, 31}, []int{34, 32, 16, 16, 17}, 2},
		{[]int{5, 10, 16, -6, 15, 11, 3}, []int{16, 22, 23, 23, 25, 3, -16}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, temperatureTrend(test.temperatureA, test.temperatureB), index)
	}
}
