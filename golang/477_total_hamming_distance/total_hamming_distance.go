/*
 * @Date: 2021-05-28 09:53:02
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-28 09:59:20
 */

package main

func totalHammingDistance(nums []int) int {
	ans, numsLen := 0, len(nums)
	for i := 0; i < 32; i++ {
		haveOne := 0
		for _, num := range nums {
			haveOne += (num >> i) & 1
		}
		ans += haveOne * (numsLen - haveOne)
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(totalHammingDistance([]int{4, 14, 2}) == 6)
}
