/*
 * @Date: 2021-07-03 10:14:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-03 11:01:25
 */

package main

import (
	"bytes"
)

func frequencySort(s string) string {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	cnt := map[byte]int{}
	maxFreq := 0
	for i := range s {
		cnt[s[i]]++
		maxFreq = max(maxFreq, cnt[s[i]])
	}

	buckets := make([][]byte, maxFreq+1)
	for ch, c := range cnt {
		buckets[c] = append(buckets[c], ch)
	}

	ans := make([]byte, 0, len(s))
	for i := maxFreq; i > 0; i-- {
		for _, ch := range buckets[i] {
			ans = append(ans, bytes.Repeat([]byte{ch}, i)...)
		}
	}
	return string(ans)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		s := "tree"
		ans := "eetr"
		assert(frequencySort(s) == ans)
	}
	{
		s := "cccaaa"
		ans := "cccaaa"
		assert(frequencySort(s) == ans)
	}
	{
		s := "Aabb"
		ans := "bbAa"
		assert(frequencySort(s) == ans)
	}
}
