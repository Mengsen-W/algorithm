/*
 * @Date: 2021-08-23 21:10:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-23 21:15:36
 */

package main

func getMaximumGenerated(n int) (ans int) {
	if n == 0 {
		return
	}
	nums := make([]int, n+1)
	nums[1] = 1
	for i := 2; i <= n; i++ {
		nums[i] = nums[i/2] + i%2*nums[i/2+1]
	}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	for _, v := range nums {
		ans = max(ans, v)
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(getMaximumGenerated(7) == 3)
	assert(getMaximumGenerated(2) == 1)
	assert(getMaximumGenerated(3) == 2)
}
