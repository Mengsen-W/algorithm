/*
 * @Date: 2022-11-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-08
 * @FilePath: /algorithm/1684_count_consistent_strings/count_consistent_strings.go
 */

package main

func countConsistentStrings(allowed string, words []string) (res int) {
	mask := 0
	for _, c := range allowed {
		mask |= 1 << (c - 'a')
	}
	for _, word := range words {
		mask1 := 0
		for _, c := range word {
			mask1 |= 1 << (c - 'a')
		}
		if mask1|mask == mask {
			res++
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
		allowed := "ab"
		words := []string{"ad", "bd", "aaab", "baa", "badab"}
		ans := 2
		assert(countConsistentStrings(allowed, words) == ans)
	}

	{
		allowed := "abc"
		words := []string{"a", "b", "c", "ab", "ac", "bc", "abc"}
		ans := 7
		assert(countConsistentStrings(allowed, words) == ans)
	}

	{
		allowed := "cad"
		words := []string{"cc", "acd", "b", "ba", "bac", "bad", "ac", "d"}
		ans := 4
		assert(countConsistentStrings(allowed, words) == ans)
	}
}
