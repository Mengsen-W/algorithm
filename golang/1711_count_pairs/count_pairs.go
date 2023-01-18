/*
 * @Date: 2021-07-07 08:54:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-07 08:59:02
 */

package main

func countPairs(deliciousness []int) (ans int) {
	maxVal := deliciousness[0]
	for _, val := range deliciousness[1:] {
		if val > maxVal {
			maxVal = val
		}
	}
	maxSum := maxVal * 2
	cnt := map[int]int{}
	for _, val := range deliciousness {
		for sum := 1; sum <= maxSum; sum <<= 1 {
			ans += cnt[sum-val]
		}
		cnt[val]++
	}
	return ans % (1e9 + 7)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		deliciousness := []int{1, 3, 5, 7, 9}
		assert(countPairs(deliciousness) == 4)
	}
	{
		deliciousness := []int{1, 1, 1, 3, 3, 3, 7}
		assert(countPairs(deliciousness) == 15)
	}
}
