/*
 * @Date: 2021-12-03 08:38:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-03 08:45:06
 */

package main

func largestSumAfterKNegations(nums []int, k int) (ans int) {
	freq := map[int]int{}
	for _, num := range nums {
		freq[num]++
		ans += num
	}
	for i := -100; i < 0 && k != 0; i++ {
		if freq[i] > 0 {
			ops := min(k, freq[i])
			ans -= i * ops * 2
			freq[-i] += ops
			k -= ops
		}
	}
	if k > 0 && k%2 == 1 && freq[0] == 0 {
		for i := 1; i <= 100; i++ {
			if freq[i] > 0 {
				ans -= i * 2
				break
			}
		}
	}
	return
}

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(largestSumAfterKNegations([]int{4, 2, 3}, 1) == 5)
	assert(largestSumAfterKNegations([]int{3, -1, 0, 2}, 3) == 6)
	assert(largestSumAfterKNegations([]int{2, -3, -1, 5, -4}, 2) == 13)
}
