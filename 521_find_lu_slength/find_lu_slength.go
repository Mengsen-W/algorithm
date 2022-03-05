/*
 * @Date: 2022-03-05 00:33:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-05 00:48:17
 * @FilePath: /algorithm/521_find_lu_slength/find_lu_slength.go
 */

package main

func findLUSlength(a, b string) int {
	if a != b {
		return func(a, b int) int {
			if b > a {
				return b
			}
			return a
		}(len(a), len(b))
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(findLUSlength("aba", "cdc") == 3)
	assert(findLUSlength("aaa", "bbb") == 3)
	assert(findLUSlength("aaa", "aaa") == -1)
}
