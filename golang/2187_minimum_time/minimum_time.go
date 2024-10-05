// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumTime(time []int, totalTrips int) int64 {
	// 二分查找下界与上界
	l, r := int64(1), int64(totalTrips)*int64(time[0])
	for _, period := range time {
		if period > int(r/int64(totalTrips)) {
			r = int64(totalTrips) * int64(period)
		}
	}
	// 二分查找寻找满足要求的最小的 t
	for l < r {
		mid := l + (r-l)/2
		if check(mid, time, totalTrips) {
			r = mid
		} else {
			l = mid + 1
		}
	}
	return l
}

// 判断 t 时间内是否可以完成 totalTrips 趟旅途
func check(t int64, time []int, totalTrips int) bool {
	var cnt int64 = 0
	for _, period := range time {
		cnt += t / int64(period)
	}
	return cnt >= int64(totalTrips)
}

func main() {
	tests := []struct {
		time       []int
		totalTrips int
		ans        int64
	}{
		{[]int{1, 2, 3}, 5, 3},
		{[]int{2}, 1, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumTime(test.time, test.totalTrips), index)
	}
}
