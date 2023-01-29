/*
 * @Date: 2023-01-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-29
 * @FilePath: /algorithm/golang/2315_count_asterisks/count_asterisks.go
 */

package main

func countAsterisks(s string) (res int) {
	valid := true
	for _, c := range s {
		if c == '|' {
			valid = !valid
		} else if c == '*' && valid {
			res++
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "l|*e*et|c**o|*de|"
		ans := 2
		assert(countAsterisks(s) == ans)
	}

	{
		s := "iamprogrammer"
		ans := 0
		assert(countAsterisks(s) == ans)
	}

	{
		s := "yo|uar|e**|b|e***au|tifu|l"
		ans := 5
		assert(countAsterisks(s) == ans)
	}
}
