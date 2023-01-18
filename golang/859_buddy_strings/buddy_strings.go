/*
 * @Date: 2021-11-23 01:10:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-23 01:11:21
 * @FilePath: /algorithm/859_buddy_strings/buddy_strings.go
 * @Description: file content
 */

package main

func buddyStrings(s, goal string) bool {
	if len(s) != len(goal) {
		return false
	}

	if s == goal {
		seen := [26]bool{}
		for _, ch := range s {
			if seen[ch-'a'] {
				return true
			}
			seen[ch-'a'] = true
		}
		return false
	}

	first, second := -1, -1
	for i := range s {
		if s[i] != goal[i] {
			if first == -1 {
				first = i
			} else if second == -1 {
				second = i
			} else {
				return false
			}
		}
	}
	return second != -1 && s[first] == goal[second] && s[second] == goal[first]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(buddyStrings("ab", "ba"))
	assert(!buddyStrings("ab", "ab"))
	assert(buddyStrings("aa", "aa"))
	assert(buddyStrings("aaaaaaabc", "aaaaaaacb"))
}
