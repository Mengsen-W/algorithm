/*
 * @Date: 2021-06-02 07:45:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-02 07:53:24
 */

package main

func checkSubarraySum(nums []int, k int) bool {
	m := len(nums)
	if m < 2 {
		return false
	}
	mp := map[int]int{0: -1}
	remainder := 0
	for i, num := range nums {
		remainder = (remainder + num) % k
		if prevIndex, has := mp[remainder]; has {
			if i-prevIndex >= 2 {
				return true
			}
		} else {
			mp[remainder] = i
		}
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(checkSubarraySum([]int{23, 2, 4, 6, 7}, 6))
	assert(checkSubarraySum([]int{23, 2, 6, 4, 7}, 6))
	assert(!checkSubarraySum([]int{23, 2, 6, 4, 7}, 13))
}
