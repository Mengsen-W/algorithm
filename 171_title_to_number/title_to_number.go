/*
 * @Date: 2021-07-30 09:49:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-30 09:58:03
 */

package main

func titleToNumber(columnTitle string) (number int) {
	for i, multiple := len(columnTitle)-1, 1; i >= 0; i-- {
		k := columnTitle[i] - 'A' + 1
		number += int(k) * multiple
		multiple *= 26
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
		s := "A"
		assert(titleToNumber(s) == 1)
	}
	{
		s := "AB"
		assert(titleToNumber(s) == 28)
	}
	{
		s := "ZY"
		assert(titleToNumber(s) == 701)
	}
}
