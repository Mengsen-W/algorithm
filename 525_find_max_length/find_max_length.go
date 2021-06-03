/*
 * @Date: 2021-06-03 08:19:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-03 08:37:13
 */

package main

func findMaxLength(nums []int) int {
	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}
	cntIdx := map[int]int{
		0: -1,
	}

	cnt, maxLen := 0, 0
	for i := 0; i < len(nums); i++ {
		if nums[i] == 0 {
			cnt += -1
		} else {
			cnt += 1
		}
		if v, ok := cntIdx[cnt]; ok {
			maxLen = max(maxLen, i-v)
		} else {
			cntIdx[cnt] = i
		}
	}

	return maxLen
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(findMaxLength([]int{0, 1}) == 2)
	assert(findMaxLength([]int{0, 1, 0}) == 2)
}
