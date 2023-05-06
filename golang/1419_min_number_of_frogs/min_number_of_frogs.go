/*
 * @Date: 2023-05-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-06
 * @FilePath: /algorithm/golang/1419_min_number_of_frogs/min_number_of_frogs.go
 */

// Package main ...
package main

func minNumberOfFrogs(croakOfFrogs string) int {
	if len(croakOfFrogs)%5 != 0 {
		return -1
	}
	res := 0
	frogNum := 0
	cnt := make([]int, 4)
	mp := map[rune]int{'c': 0, 'r': 1, 'o': 2, 'a': 3, 'k': 4}
	for _, c := range croakOfFrogs {
		t := mp[c]
		if t == 0 {
			cnt[t]++
			frogNum++
			if frogNum > res {
				res = frogNum
			}
		} else {
			if cnt[t-1] == 0 {
				return -1
			}
			cnt[t-1]--
			if t == 4 {
				frogNum--
			} else {
				cnt[t]++
			}
		}
	}
	if frogNum > 0 {
		return -1
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
		croakOfFrogs := "croakcroak"
		ans := 1
		assert(minNumberOfFrogs(croakOfFrogs) == ans)
	}

	{
		croakOfFrogs := "crcoakroak"
		ans := 2
		assert(minNumberOfFrogs(croakOfFrogs) == ans)
	}

	{
		croakOfFrogs := "croakcrook"
		ans := -1
		assert(minNumberOfFrogs(croakOfFrogs) == ans)
	}
}
