/*
 * @Date: 2023-02-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-25
 * @FilePath: /algorithm/golang/1247_minimum_swap/minimum_swap.go
 */

package main

func minimumSwap(s1 string, s2 string) int {
	xy, yx := 0, 0
	n := len(s1)
	for i := 0; i < n; i++ {
		a, b := s1[i], s2[i]
		if a == 'x' && b == 'y' {
			xy++
		}
		if a == 'y' && b == 'x' {
			yx++
		}
	}
	if (xy+yx)%2 == 1 {
		return -1
	}
	return xy/2 + yx/2 + xy%2 + yx%2
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		s1 := "xx"
		s2 := "yy"
		ans := 1
		assert(minimumSwap(s1, s2) == ans)
	}

	{
		s1 := "xy"
		s2 := "yx"
		ans := 2
		assert(minimumSwap(s1, s2) == ans)
	}

	{
		s1 := "xx"
		s2 := "xy"
		ans := -1
		assert(minimumSwap(s1, s2) == ans)
	}

	{
		s1 := "xxyyxyxyxx"
		s2 := "xyyxyxxxyx"
		ans := 4
		assert(minimumSwap(s1, s2) == ans)
	}
}
