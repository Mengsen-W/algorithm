/*
 * @Date: 2022-10-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-31
 * @FilePath: /algorithm/481_magical_string/magical_string.go
 */

package main

func magicalString(n int) int {
	if n < 4 {
		return 1
	}
	s := make([]byte, n)
	copy(s, "122")
	res := 1
	i, j := 2, 3
	for j < n {
		size := s[i] - '0'
		num := 3 - (s[j-1] - '0')
		for size > 0 && j < n {
			s[j] = '0' + num
			if num == 1 {
				res++
			}
			j++
			size--
		}
		i++
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(magicalString(6) == 3)
	assert(magicalString(1) == 1)
}
