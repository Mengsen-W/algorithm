/*
 * @Date: 2021-08-17 09:24:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-17 09:28:58
 */

package main

func checkRecord(s string) bool {
	absents, lates := 0, 0
	for _, ch := range s {
		if ch == 'A' {
			absents++
			if absents >= 2 {
				return false
			}
		}
		if ch == 'L' {
			lates++
			if lates >= 3 {
				return false
			}
		} else {
			lates = 0
		}
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
		s := "PPALLP"
		assert(checkRecord(s))
	}
	{
		s := "PPALLL"
		assert(!checkRecord(s))
	}
}
