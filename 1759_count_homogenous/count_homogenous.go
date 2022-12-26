/*
 * @Date: 2022-12-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-26
 * @FilePath: /algorithm/1759_count_homogenous/count_homogenous.go
 */
package main

func countHomogenous(s string) (res int) {
	const mod int = 1e9 + 7
	prev := rune(s[0])
	cnt := 0
	for _, c := range s {
		if c == prev {
			cnt++
		} else {
			res += (cnt + 1) * cnt / 2
			cnt = 1
			prev = c
		}
	}
	res += (cnt + 1) * cnt / 2
	return res % mod
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "abbcccaa"
		ans := 13
		assert(countHomogenous(s) == ans)
	}

	{
		s := "xy"
		ans := 2
		assert(countHomogenous(s) == ans)
	}

	{
		s := "zzzzz"
		ans := 15
		assert(countHomogenous(s) == ans)
	}
}
