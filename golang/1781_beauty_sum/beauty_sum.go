/*
 * @Date: 2022-12-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-12
 * @FilePath: /algorithm/1781_beauty_sum/beauty_sum.go
 */

package main

func beautySum(s string) (ans int) {

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	for i := range s {
		cnt := [26]int{}
		maxFreq := 0
		for _, c := range s[i:] {
			cnt[c-'a']++
			maxFreq = max(maxFreq, cnt[c-'a'])
			minFreq := len(s)
			for _, c := range cnt {
				if c > 0 {
					minFreq = min(minFreq, c)
				}
			}
			ans += maxFreq - minFreq
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

	{
		s := "aabcb"
		ans := 5
		assert(beautySum(s) == ans)
	}

	{
		s := "aabcbaa"
		ans := 17
		assert(beautySum(s) == ans)
	}
}
