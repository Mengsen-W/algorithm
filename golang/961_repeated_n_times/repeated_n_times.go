/*
 * @Date: 2022-05-21 21:41:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-21 21:47:56
 * @FilePath: /algorithm/961_repeated_n_times/repeated_n_times.go
 */

package main

import "math/rand"

func repeatedNTimes(nums []int) int {
	n := len(nums)
	for {
		x, y := rand.Intn(n), rand.Intn(n)
		if x != y && nums[x] == nums[y] {
			return nums[x]
		}
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(repeatedNTimes([]int{1, 2, 3, 3}) == 3)
	assert(repeatedNTimes([]int{2, 1, 2, 5, 3, 2}) == 2)
	assert(repeatedNTimes([]int{5, 1, 5, 2, 5, 3, 5, 4}) == 5)
}
