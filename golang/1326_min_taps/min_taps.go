/*
 * @Date: 2023-02-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-21
 * @FilePath: /algorithm/golang/1326_min_taps/min_taps.go
 */

package main

func minTaps(n int, ranges []int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	rightMost := make([]int, n+1)
	for i := range rightMost {
		rightMost[i] = i
	}
	for i, r := range ranges {
		start := max(0, i-r)
		end := min(n, i+r)
		rightMost[start] = max(rightMost[start], end)
	}

	last, ret, pre := 0, 0, 0
	for i := 0; i < n; i++ {
		last = max(last, rightMost[i])
		if i == last {
			return -1
		}
		if i == pre {
			ret++
			pre = last
		}
	}
	return ret
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		n := 5
		ranges := []int{3, 4, 1, 1, 0, 0}
		ans := 1
		assert(minTaps(n, ranges) == ans)
	}

	{
		n := 3
		ranges := []int{0, 0, 0, 0}
		ans := -1
		assert(minTaps(n, ranges) == ans)
	}
}
