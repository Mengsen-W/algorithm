/*
 * @Date: 2022-06-16 09:46:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-16 09:51:15
 * @FilePath: /algorithm/719_smallest_distance_pair/smallest_distance_pair.go
 */

package main

import "sort"

func smallestDistancePair(nums []int, k int) int {
	sort.Ints(nums)
	return sort.Search(nums[len(nums)-1]-nums[0], func(mid int) bool {
		cnt, i := 0, 0
		for j, num := range nums {
			for num-nums[i] > mid {
				i++
			}
			cnt += j - i
		}
		return cnt >= k
	})
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(smallestDistancePair([]int{1, 3, 1}, 1) == 0)
	assert(smallestDistancePair([]int{1, 1, 1}, 2) == 0)
	assert(smallestDistancePair([]int{1, 6, 1}, 3) == 5)
}
