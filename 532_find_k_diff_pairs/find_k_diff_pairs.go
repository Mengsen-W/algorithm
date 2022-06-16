/*
 * @Date: 2022-06-16 09:53:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-16 10:09:50
 * @FilePath: /algorithm/532_find_k_diff_pairs/find_k_diff_pairs.go
 */

package main

import "sort"

func findPairs(nums []int, k int) (ans int) {
	sort.Ints(nums)
	y, n := 0, len(nums)
	for x, num := range nums {
		if x == 0 || num != nums[x-1] {
			for y < n && (nums[y] < num+k || y <= x) {
				y++
			}
			if y < n && nums[y] == num+k {
				ans++
			}
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
	assert(findPairs([]int{3, 1, 4, 1, 5}, 2) == 2)
	assert(findPairs([]int{1, 2, 3, 4, 5}, 1) == 4)
	assert(findPairs([]int{1, 3, 1, 5, 4}, 0) == 1)
}
