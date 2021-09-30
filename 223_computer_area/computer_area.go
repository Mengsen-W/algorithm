/*
 * @Date: 2021-09-30 09:16:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-30 09:21:27
 */

package main

func computeArea(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2 int) int {
	area1 := (ax2 - ax1) * (ay2 - ay1)
	area2 := (bx2 - bx1) * (by2 - by1)
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
	overlapWidth := min(ax2, bx2) - max(ax1, bx1)
	overlapHeight := min(ay2, by2) - max(ay1, by1)
	overlapArea := max(overlapWidth, 0) * max(overlapHeight, 0)
	return area1 + area2 - overlapArea
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		ax1 := -3
		ay1 := 0
		ax2 := 3
		ay2 := 4
		bx1 := 0
		by1 := -1
		bx2 := 9
		by2 := 2
		ans := 45
		assert(computeArea(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) == ans)
	}
	{
		ax1 := -2
		ay1 := -2
		ax2 := 2
		ay2 := 2
		bx1 := -2
		by1 := -2
		bx2 := 2
		by2 := 2
		ans := 16
		assert(computeArea(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) == ans)
	}
}
