/*
 * @Date: 2022-06-07 09:54:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-07 10:20:09
 * @FilePath: /algorithm/875_min_eating_speed/min_eating_speed.go
 */

package main

import "sort"

func minEatingSpeed(piles []int, h int) int {
	max := 0
	for _, pile := range piles {
		if pile > max {
			max = pile
		}
	}
	return 1 + sort.Search(max-1, func(speed int) bool {
		speed++
		time := 0
		for _, pile := range piles {
			time += (pile + speed - 1) / speed
		}
		return time <= h
	})
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(minEatingSpeed([]int{3, 6, 7, 11}, 8) == 4)
	assert(minEatingSpeed([]int{30, 11, 23, 4, 20}, 5) == 30)
	assert(minEatingSpeed([]int{30, 11, 23, 4, 20}, 6) == 23)
}
