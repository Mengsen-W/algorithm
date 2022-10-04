/*
 * @Date: 2022-10-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-04
 * @FilePath: /algorithm/921_min_add_to_make_valid/min_add_to_make_valid.go
 */

package main

func minAddToMakeValid(s string) (ans int) {
	cnt := 0
	for _, c := range s {
		if c == '(' {
			cnt++
		} else if cnt > 0 {
			cnt--
		} else {
			ans++
		}
	}
	return ans + cnt
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minAddToMakeValid("())") == 1)
	assert(minAddToMakeValid("(((") == 3)
}
