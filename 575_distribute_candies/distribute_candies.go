/*
 * @Date: 2021-11-01 01:10:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-01 01:17:30
 */

package main

func distributeCandies(candyType []int) int {
	set := map[int]struct{}{}
	for _, t := range candyType {
		set[t] = struct{}{}
	}
	ans := len(candyType) / 2
	if len(set) < ans {
		ans = len(set)
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(distributeCandies([]int{1, 1, 2, 2, 3, 3}) == 3)
	assert(distributeCandies([]int{1, 1, 2, 3}) == 2)
}
