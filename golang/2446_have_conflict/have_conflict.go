/*
 * @Date: 2023-05-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-17
 * @FilePath: /algorithm/golang/2446_have_conflict/have_conflict.go
 */

// Package main ...
package main

func haveConflict(event1 []string, event2 []string) bool {
	return !(event1[1] < event2[0] || event2[1] < event1[0])
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		event1 := []string{"01:15", "02:00"}
		event2 := []string{"02:00", "03:00"}
		ans := true
		assert(haveConflict(event1, event2) == ans)
	}

	{
		event1 := []string{"01:00", "02:00"}
		event2 := []string{"01:20", "03:00"}
		ans := true
		assert(haveConflict(event1, event2) == ans)
	}

	{
		event1 := []string{"10:00", "11:00"}
		event2 := []string{"14:00", "15:00"}
		ans := false
		assert(haveConflict(event1, event2) == ans)
	}
}
