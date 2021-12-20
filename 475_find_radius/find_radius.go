/*
 * @Date: 2021-12-20 00:46:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-20 01:01:18
 */

package main

import "sort"

func findRadius(houses, heaters []int) (ans int) {
	sort.Ints(houses)
	sort.Ints(heaters)
	j := 0
	for _, house := range houses {
		dis := abs(house - heaters[j])
		for j+1 < len(heaters) && abs(house-heaters[j]) >= abs(house-heaters[j+1]) {
			j++
			if abs(house-heaters[j]) < dis {
				dis = abs(house - heaters[j])
			}
		}
		if dis > ans {
			ans = dis
		}
	}
	return
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	assert(findRadius([]int{1, 2, 3}, []int{2}), 1)
	assert(findRadius([]int{1, 2, 3, 4}, []int{1, 4}), 1)
	assert(findRadius([]int{1, 5}, []int{2}), 3)
}
