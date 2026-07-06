// Package main ...
package main

import (
	"fmt"
	"sort"
)

func removeCoveredIntervals(intervals [][]int) int {
	n := len(intervals)
	sort.Slice(intervals, func(i, j int) bool {
		if intervals[i][0] != intervals[j][0] {
			return intervals[i][0] < intervals[j][0]
		}
		return intervals[i][1] > intervals[j][1]
	})
	ans := n
	rmax := intervals[0][1]
	for i := 1; i < n; i++ {
		if intervals[i][1] <= rmax {
			ans--
		} else {
			if intervals[i][1] > rmax {
				rmax = intervals[i][1]
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		intervals [][]int
		expected  int
	}{
		{[][]int{{1, 4}, {3, 6}, {2, 8}}, 2},
	}

	for _, test := range tests {
		if output := removeCoveredIntervals(test.intervals); output != test.expected {
			fmt.Println("test failed", test.intervals, output, test.expected)
		}
	}
}
