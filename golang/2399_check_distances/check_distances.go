/*
 * @Date: 2023-04-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-09
 * @FilePath: /algorithm/golang/2399_check_distances/check_distances.go
 */

// Package main ...
package main

func checkDistances(s string, distance []int) bool {
	firstIndex := make([]int, 26)
	for i := 0; i < len(s); i++ {
		idx := s[i] - 'a'
		if firstIndex[idx] != 0 && i-firstIndex[idx] != distance[idx] {
			return false
		}
		firstIndex[idx] = i + 1
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "abaccb"
		distance := []int{1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
		ans := true
		assert(checkDistances(s, distance) == ans)
	}

	{
		s := "aa"
		distance := []int{1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
		ans := false
		assert(checkDistances(s, distance) == ans)
	}
}
