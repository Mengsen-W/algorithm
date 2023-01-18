/*
 * @Date: 2021-12-29 01:26:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-29 01:44:28
 */

package main

func countQuadruplets(nums []int) (ans int) {
	cnt := map[int]int{}
	for b := len(nums) - 3; b >= 1; b-- {
		for _, x := range nums[b+2:] {
			cnt[x-nums[b+1]]++
		}
		for _, x := range nums[:b] {
			if sum := x + nums[b]; cnt[sum] > 0 {
				ans += cnt[sum]
			}
		}
	}
	return
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	assert(countQuadruplets([]int{1, 2, 3, 6}), 1)
	assert(countQuadruplets([]int{3, 3, 6, 4, 5}), 0)
	assert(countQuadruplets([]int{1, 1, 1, 3, 5}), 4)
}
