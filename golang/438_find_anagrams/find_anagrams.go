/*
 * @Date: 2021-11-28 02:17:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-28 02:25:48
 */

package main

import "reflect"

func findAnagrams(s, p string) (ans []int) {
	sLen, pLen := len(s), len(p)
	if sLen < pLen {
		return
	}

	count := [26]int{}
	for i, ch := range p {
		count[s[i]-'a']++
		count[ch-'a']--
	}

	differ := 0
	for _, c := range count {
		if c != 0 {
			differ++
		}
	}
	if differ == 0 {
		ans = append(ans, 0)
	}

	for i, ch := range s[:sLen-pLen] {
		if count[ch-'a'] == 1 {
			differ--
		} else if count[ch-'a'] == 0 {
			differ++
		}
		count[ch-'a']--

		if count[s[i+pLen]-'a'] == -1 {
			differ--
		} else if count[s[i+pLen]-'a'] == 0 {
			differ++
		}
		count[s[i+pLen]-'a']++

		if differ == 0 {
			ans = append(ans, i+1)
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(findAnagrams("cbaebabacd", "abc"), []int{0, 6})
	assert(findAnagrams("abab", "ab"), []int{0, 1, 2})
}
