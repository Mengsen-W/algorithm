/*
 * @Date: 2021-12-04 05:41:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-04 05:43:13
 */

package main

func canConstruct(ransomNote, magazine string) bool {
	if len(ransomNote) > len(magazine) {
		return false
	}
	cnt := [26]int{}
	for _, ch := range magazine {
		cnt[ch-'a']++
	}
	for _, ch := range ransomNote {
		cnt[ch-'a']--
		if cnt[ch-'a'] < 0 {
			return false
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(canConstruct("a", "b") == false)
	assert(canConstruct("aa", "ab") == false)
	assert(canConstruct("aa", "aab") == true)
}
