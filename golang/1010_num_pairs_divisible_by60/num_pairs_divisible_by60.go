/*
 * @Date: 2023-05-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-07
 * @FilePath: /algorithm/golang/1010_num_pairs_divisible_by60/num_pairs_divisible_by60.go
 */

// Package main ...
package main

func numPairsDivisibleBy60(time []int) int {
	cnt := make([]int, 60)
	for _, t := range time {
		cnt[t%60]++
	}
	var res int
	for i := 1; i < 30; i++ {
		res += cnt[i] * cnt[60-i]
	}
	res += cnt[0]*(cnt[0]-1)/2 + cnt[30]*(cnt[30]-1)/2
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		time := []int{30, 20, 150, 100, 40}
		ans := 3
		assert(numPairsDivisibleBy60(time) == ans)
	}

	{
		time := []int{60, 60, 60}
		ans := 3
		assert(numPairsDivisibleBy60(time) == ans)
	}
}
