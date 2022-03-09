/*
 * @Date: 2022-03-09 00:34:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-09 01:00:37
 * @FilePath: /algorithm/798_best_rotation/best_rotation.go
 */

package main

func bestRotation(nums []int) int {
	n := len(nums)
	diffs := make([]int, n)
	for i, num := range nums {
		low := (i + 1) % n
		high := (i - num + n + 1) % n
		diffs[low]++
		diffs[high]--
		if low >= high {
			diffs[0]++
		}
	}
	score, maxScore, idx := 0, 0, 0
	for i, diff := range diffs {
		score += diff
		if score > maxScore {
			maxScore, idx = score, i
		}
	}
	return idx
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(bestRotation([]int{2, 3, 1, 4, 0}) == 3)
	assert(bestRotation([]int{1, 3, 0, 2, 4}) == 0)
}
