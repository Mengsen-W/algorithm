/*
 * @Date: 2022-12-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-13
 * @FilePath: /algorithm/1832_check_if_pangram/check_if_pangram.go
 */

package main

func checkIfPangram(sentence string) bool {
	state := 0
	for _, c := range sentence {
		state |= 1 << (c - 'a')
	}
	return state == 1<<26-1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(checkIfPangram("thequickbrownfoxjumpsoverthelazydog"))
	assert(!checkIfPangram("leetcode"))
}
