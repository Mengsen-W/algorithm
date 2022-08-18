/*
 * @Date: 2022-08-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-18
 * @FilePath: /algorithm/1224_max_equal_freq/max_equal_freq.go
 */

package main

func maxEqualFreq(nums []int) (ans int) {
	freq := map[int]int{}
	count := map[int]int{}
	maxFreq := 0
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for i, num := range nums {
		if count[num] > 0 {
			freq[count[num]]--
		}
		count[num]++
		maxFreq = max(maxFreq, count[num])
		freq[count[num]]++
		if maxFreq == 1 ||
			freq[maxFreq]*maxFreq+freq[maxFreq-1]*(maxFreq-1) == i+1 && freq[maxFreq] == 1 ||
			freq[maxFreq]*maxFreq+1 == i+1 && freq[1] == 1 {
			ans = max(ans, i+1)
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
	assert(maxEqualFreq([]int{2, 2, 1, 1, 5, 3, 3, 5}) == 7)
	assert(maxEqualFreq([]int{1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5}) == 13)
}
