/*
 * @Date: 2022-01-19 01:48:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-19 01:51:06
 */

package main

func containsNearbyDuplicate(nums []int, k int) bool {
	set := map[int]struct{}{}
	for i, num := range nums {
		if i > k {
			delete(set, nums[i-k-1])
		}
		if _, ok := set[num]; ok {
			return true
		}
		set[num] = struct{}{}
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(containsNearbyDuplicate([]int{1, 2, 3, 1}, 3) == true)
	assert(containsNearbyDuplicate([]int{1, 0, 1, 1}, 1) == true)
	assert(containsNearbyDuplicate([]int{1, 2, 3, 1, 2, 3}, 2) == false)
}
