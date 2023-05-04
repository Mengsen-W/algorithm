/*
 * @Date: 2023-05-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-04
 * @FilePath: /algorithm/golang/2106_max_total_fruits/max_total_fruits.go
 */

// Package main ...
package main

func maxTotalFruits(fruits [][]int, startPos int, k int) int {
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

	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	left := 0
	right := 0
	n := len(fruits)
	sum := 0
	ans := 0

	step := func(left int, right int) int {
		if fruits[right][0] <= startPos {
			return startPos - fruits[left][0]
		} else if fruits[left][0] >= startPos {
			return fruits[right][0] - startPos
		} else {
			return min(abs(startPos-fruits[right][0]), abs(startPos-fruits[left][0])) + fruits[right][0] - fruits[left][0]
		}
	}
	// 每次固定住窗口右边界
	for right < n {
		sum += fruits[right][1]
		// 移动左边界
		for left <= right && step(left, right) > k {
			sum -= fruits[left][1]
			left++
		}
		ans = max(ans, sum)
		right++
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		fruits := [][]int{{2, 8}, {6, 3}, {8, 6}}
		startPos := 5
		k := 4
		ans := 9
		assert(maxTotalFruits(fruits, startPos, k) == ans)
	}

	{
		fruits := [][]int{{0, 9}, {4, 1}, {5, 7}, {6, 2}, {7, 4}, {10, 9}}
		startPos := 5
		k := 4
		ans := 14
		assert(maxTotalFruits(fruits, startPos, k) == ans)
	}

	{
		fruits := [][]int{{0, 3}, {6, 4}, {8, 5}}
		startPos := 3
		k := 2
		ans := 0
		assert(maxTotalFruits(fruits, startPos, k) == ans)
	}
}
