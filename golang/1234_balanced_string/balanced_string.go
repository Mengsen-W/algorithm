/*
 * @Date: 2023-02-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-13
 * @FilePath: /algorithm/golang/1234_balanced_string/balanced_string.go
 */

package main

func balancedString(s string) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	cnt := map[byte]int{}
	for _, c := range s {
		cnt[byte(c)]++
	}
	partial := len(s) / 4
	check := func() bool {
		if cnt['Q'] > partial ||
			cnt['W'] > partial ||
			cnt['E'] > partial ||
			cnt['R'] > partial {
			return false
		}
		return true
	}

	if check() {
		return 0
	}

	res := len(s)
	r := 0
	for l, c := range s {
		for r < len(s) && !check() {
			cnt[s[r]]--
			r++
		}
		if !check() {
			break
		}
		res = min(res, r-l)
		cnt[byte(c)]++
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
		s := "QWER"
		ans := 0
		assert(balancedString(s) == ans)
	}

	{
		s := "QQWE"
		ans := 1
		assert(balancedString(s) == ans)
	}

	{
		s := "QQQW"
		ans := 2
		assert(balancedString(s) == ans)
	}

	{
		s := "QQQQ"
		ans := 3
		assert(balancedString(s) == ans)
	}

}
