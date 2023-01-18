/*
 * @Date: 2021-11-06 01:09:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-06 01:16:15
 */

package main

func missingNumber(nums []int) int {
	n := len(nums)
	total := n * (n + 1) / 2
	arrSum := 0
	for _, v := range nums {
		arrSum += v
	}
	return total - arrSum
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(missingNumber([]int{3, 0, 1}) == 2)
	assert(missingNumber([]int{9, 6, 4, 2, 3, 5, 7, 0, 1}) == 8)
	assert(missingNumber([]int{0}) == 1)
	assert(missingNumber([]int{0, 1}) == 2)

}
