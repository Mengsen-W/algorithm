/*
 * @Date: 2022-06-13 09:52:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-13 10:05:30
 * @FilePath: /algorithm/1051_height_checker/height_checker.go
 */

package main

func heightChecker(heights []int) (ans int) {
	cnt := [101]int{}
	for _, v := range heights {
		cnt[v]++
	}

	idx := 0
	for i, c := range cnt {
		for ; c > 0; c-- {
			if heights[idx] != i {
				ans++
			}
			idx++
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(heightChecker([]int{1, 1, 4, 2, 1, 3}) == 3)
	assert(heightChecker([]int{5, 1, 2, 3, 4}) == 5)
	assert(heightChecker([]int{1, 2, 3, 4, 5}) == 0)
}
