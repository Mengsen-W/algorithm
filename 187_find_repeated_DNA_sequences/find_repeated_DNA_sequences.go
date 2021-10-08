/*
 * @Date: 2021-10-08 00:13:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-08 00:16:13
 */

package main

import "reflect"

const L = 10

var bin = map[byte]int{'A': 0, 'C': 1, 'G': 2, 'T': 3}

func findRepeatedDnaSequences(s string) (ans []string) {
	n := len(s)
	if n <= L {
		return
	}
	x := 0
	for _, ch := range s[:L-1] {
		x = x<<2 | bin[byte(ch)]
	}
	cnt := map[int]int{}
	for i := 0; i <= n-L; i++ {
		x = (x<<2 | bin[s[i+L-1]]) & (1<<(L*2) - 1)
		cnt[x]++
		if cnt[x] == 2 {
			ans = append(ans, s[i:i+L])
		}
	}
	return ans
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		s := "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
		ans := []string{"AAAAACCCCC", "CCCCCAAAAA"}
		assert(findRepeatedDnaSequences(s), ans)
	}
	{
		s := "AAAAAAAAAAAAA"
		ans := []string{"AAAAAAAAAA"}
		assert(findRepeatedDnaSequences(s), ans)
	}
}
