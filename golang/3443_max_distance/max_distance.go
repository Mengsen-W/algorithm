// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxDistance(s string, k int) int {
	ans := 0
	north, south, east, west := 0, 0, 0, 0
	count := func(drt1, drt2, times int) int {
		return int(math.Abs(float64(drt1-drt2))) + times*2
	} // Calculate modified Manhattan distance

	for _, it := range s {
		switch it {
		case 'N':
			north++
		case 'S':
			south++
		case 'E':
			east++
		case 'W':
			west++
		}
		times1 := min(min(north, south), k)      // modification times for N and S
		times2 := min(min(east, west), k-times1) // modification times for E and W
		current := count(north, south, times1) + count(east, west, times2)
		if current > ans {
			ans = current
		}
	}

	return ans
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans int
	}{
		{"NWSE", 1, 3},
		{"NSWWEW", 3, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxDistance(test.s, test.k), index)
	}
}
