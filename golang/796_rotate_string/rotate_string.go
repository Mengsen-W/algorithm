/*
 * @Date: 2022-04-07 01:43:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-07 01:50:13
 * @FilePath: /algorithm/796_rotate_string/rotate_string.go
 */

package main

import "strings"

func rotateString(s, goal string) bool {
	return len(s) == len(goal) && strings.Contains(s+s, goal)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(rotateString("abcde", "cdeab") == true)
	assert(rotateString("abcde", "abced") == false)
}
