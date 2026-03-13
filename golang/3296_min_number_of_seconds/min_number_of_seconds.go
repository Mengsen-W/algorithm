// Package main ...
package main

import (
	"fmt"
	"math"
)

func minNumberOfSeconds(mountainHeight int, workerTimes []int) int64 {
	const eps = 1e-7
	maxWorkerTimes := 0
	for _, t := range workerTimes {
		if t > maxWorkerTimes {
			maxWorkerTimes = t
		}
	}

	l := int64(1)
	r := int64(maxWorkerTimes) * int64(mountainHeight) * int64(mountainHeight+1) / 2
	var ans int64 = 0

	for l <= r {
		mid := (l + r) / 2
		var cnt int64 = 0

		for _, t := range workerTimes {
			work := mid / int64(t)
			// 求最大的 k 满足 1+2+...+k <= work
			k := int64((-1.0+math.Sqrt(1+float64(work)*8))/2 + eps)
			cnt += k
		}
		if cnt >= int64(mountainHeight) {
			ans = mid
			r = mid - 1
		} else {
			l = mid + 1
		}
	}

	return ans
}

func main() {
	tests := []struct {
		mountainHeight int
		workerTimes    []int
		ans            int64
	}{
		{4, []int{2, 1, 1}, 3},
		{10, []int{3, 2, 2, 4}, 12},
	}

	for _, test := range tests {
		ans := minNumberOfSeconds(test.mountainHeight, test.workerTimes)
		if ans != test.ans {
			panic(fmt.Errorf("test %v failed, got %v", test, ans))
		}
	}
}
