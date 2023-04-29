/*
 * @Date: 2023-04-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-29
 * @FilePath: /algorithm/golang/2423_equal_frequency/equal_frequency.go
 */

// Package main ...
package main

func equalFrequency(word string) bool {
	charCount := [26]int{}
	for _, c := range word {
		charCount[c-'a'] += 1
	}
	freqCount := make(map[int]int)
	for _, c := range charCount {
		if c > 0 {
			freqCount[c] += 1
		}
	}
	for _, c := range charCount {
		if c == 0 {
			continue
		}
		freqCount[c] -= 1
		if freqCount[c] == 0 {
			delete(freqCount, c)
		}
		if c-1 > 0 {
			freqCount[c-1] += 1
		}
		if len(freqCount) == 1 {
			return true
		}
		if c-1 > 0 {
			freqCount[c-1] -= 1
			if freqCount[c-1] == 0 {
				delete(freqCount, c-1)
			}
		}
		freqCount[c] += 1
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(equalFrequency("abcc") == true)
	assert(equalFrequency("aazz") == false)
}
