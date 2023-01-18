/*
 * @Date: 2021-07-23 18:50:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-23 18:56:16
 */

package main

func isCovered(ranges [][]int, left, right int) bool {
	diff := [52]int{} // 差分数组
	for _, r := range ranges {
		diff[r[0]]++
		diff[r[1]+1]--
	}
	cnt := 0 // 前缀和
	for i := 1; i <= 50; i++ {
		cnt += diff[i]
		if cnt <= 0 && left <= i && i <= right {
			return false
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		ranges := [][]int{{1, 2}, {3, 4}, {5, 6}}
		left := 2
		right := 5
		assert(isCovered(ranges, left, right))
	}
	{
		ranges := [][]int{{10, 20}, {10, 20}}
		left := 21
		right := 21
		assert(!isCovered(ranges, left, right))
	}
}
