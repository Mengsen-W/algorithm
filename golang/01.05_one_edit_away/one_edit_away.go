/*
 * @Date: 2022-05-13 09:27:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-13 09:34:37
 * @FilePath: /algorithm/01.05_one_edit_away/one_edit_away.go
 */

package main

func oneEditAway(first, second string) bool {
	m, n := len(first), len(second)
	if m < n {
		return oneEditAway(second, first)
	}
	if m-n > 1 {
		return false
	}
	for i, ch := range second {
		if first[i] != byte(ch) {
			if m == n {
				return first[i+1:] == second[i+1:]
			}
			return first[i+1:] == second[i:]
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
	assert(oneEditAway("pale", "ple"))
	assert(!oneEditAway("pales", "pal"))
}
