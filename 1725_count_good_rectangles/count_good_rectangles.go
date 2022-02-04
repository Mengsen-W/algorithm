/*
 * @Date: 2022-02-04 00:55:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-04 01:02:46
 */

package main

func countGoodRectangles(rectangles [][]int) (ans int) {
	maxLen := 0
	for _, rect := range rectangles {
		k := min(rect[0], rect[1])
		if k == maxLen {
			ans++
		} else if k > maxLen {
			maxLen, ans = k, 1
		}
	}
	return
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(countGoodRectangles(
		[][]int{{5, 8}, {3, 9}, {5, 12}, {16, 5}}) == 3)

	assert(countGoodRectangles(
		[][]int{{2, 3}, {3, 7}, {4, 3}, {3, 7}}) == 3)
}
