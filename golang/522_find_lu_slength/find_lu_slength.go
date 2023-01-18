/*
 * @Date: 2022-06-27
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-27
 * @FilePath: /algorithm/522_find_lu_slength/find_lu_slength.go
 */

package main

func isSubseq(s, t string) bool {
	ptS := 0
	for ptT := range t {
		if s[ptS] == t[ptT] {
			if ptS++; ptS == len(s) {
				return true
			}
		}
	}
	return false
}

func findLUSlength(strs []string) int {
	ans := -1
next:
	for i, s := range strs {
		for j, t := range strs {
			if i != j && isSubseq(s, t) {
				continue next
			}
		}
		if len(s) > ans {
			ans = len(s)
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		strs := []string{"aba", "cdc", "eae"}
		assert(findLUSlength(strs) == 3)
	}

	{
		strs := []string{"aaa", "aaa", "aa"}
		assert(findLUSlength(strs) == -1)
	}
}
