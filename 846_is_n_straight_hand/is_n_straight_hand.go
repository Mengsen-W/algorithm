/*
 * @Date: 2021-12-30 01:18:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-30 01:40:53
 */

package main

import "sort"

func isNStraightHand(hand []int, groupSize int) bool {
	if len(hand)%groupSize > 0 {
		return false
	}
	sort.Ints(hand)
	cnt := map[int]int{}
	for _, num := range hand {
		cnt[num]++
	}
	for _, x := range hand {
		if cnt[x] == 0 {
			continue
		}
		for num := x; num < x+groupSize; num++ {
			if cnt[num] == 0 {
				return false
			}
			cnt[num]--
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
	assert(isNStraightHand([]int{1, 2, 3, 6, 2, 3, 4, 7, 8}, 3) == true)
	assert(isNStraightHand([]int{1, 2, 3, 4, 5}, 4) == false)
}
