/*
 * @Date: 2023-03-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-06
 * @FilePath: /algorithm/golang/1653_minimum_deletions/minimum_deletions.go
 */

package main

func minimumDeletions(s string) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	leftb := 0
	righta := 0
	for _, c := range s {
		if c == 'a' {
			righta++
		}
	}
	res := righta
	for _, c := range s {
		if c == 'a' {
			righta--
		} else {
			leftb++
		}
		res = min(res, leftb+righta)
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "aababbab"
		ans := 2
		assert(minimumDeletions(s) == ans)
	}

	{
		s := "bbaaaaabb"
		ans := 2
		assert(minimumDeletions(s) == ans)
	}
}
