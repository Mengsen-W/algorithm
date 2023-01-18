/*
 * @Date: 2022-10-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-02
 * @FilePath: /algorithm/777_can_transform/can_transform.go
 */

package main

func canTransform(start, end string) bool {
	i, j, n := 0, 0, len(start)
	for i < n && j < n {
		for i < n && start[i] == 'X' {
			i++
		}
		for j < n && end[j] == 'X' {
			j++
		}
		if i < n && j < n {
			if start[i] != end[j] {
				return false
			}
			c := start[i]
			if c == 'L' && i < j || c == 'R' && i > j {
				return false
			}
			i++
			j++
		}
	}
	for i < n {
		if start[i] != 'X' {
			return false
		}
		i++
	}
	for j < n {
		if end[j] != 'X' {
			return false
		}
		j++
	}
	return true
}

func main() {
	func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}(canTransform("RXXLRXRXL", "XRLXXRRLX"))
}
