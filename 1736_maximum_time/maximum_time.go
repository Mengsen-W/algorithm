/*
 * @Date: 2021-07-24 13:31:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-24 13:38:44
 */

package main

func maximumTime(time string) string {
	t := []byte(time)
	if t[0] == '?' {
		if '4' <= t[1] && t[1] <= '9' {
			t[0] = '1'
		} else {
			t[0] = '2'
		}
	}
	if t[1] == '?' {
		if t[0] == '2' {
			t[1] = '3'
		} else {
			t[1] = '9'
		}
	}
	if t[3] == '?' {
		t[3] = '5'
	}
	if t[4] == '?' {
		t[4] = '9'
	}
	return string(t)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		time := "2?:?0"
		assert(maximumTime(time) == "23:50")
	}

	{
		time := "0?:3?"
		assert(maximumTime(time) == "09:39")
	}

	{
		time := "1?:22"
		assert(maximumTime(time) == "19:22")
	}
}
