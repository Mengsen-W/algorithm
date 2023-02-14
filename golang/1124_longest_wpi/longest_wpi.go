/*
 * @Date: 2023-02-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-14
 * @FilePath: /algorithm/golang/1124_longest_wpi/longest_wpi.go
 */

package main

func longestWPI(hours []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	n := len(hours)
	ump := map[int]int{}
	s := 0
	res := 0
	for i := 0; i < n; i++ {
		if hours[i] > 8 {
			s++
		} else {
			s--
		}

		if s > 0 {
			res = max(res, i+1)
		} else {
			if value, ok := ump[s-1]; ok {
				res = max(res, i-value)
			}
		}
		if _, ok := ump[s]; !ok {
			ump[s] = i
		}
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
		hours := []int{9, 9, 6, 0, 6, 6, 9}
		ans := 3
		assert(longestWPI(hours) == ans)
	}

	{
		hours := []int{6, 6, 6}
		ans := 0
		assert(longestWPI(hours) == ans)
	}
}
