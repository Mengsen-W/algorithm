/*
 * @Date: 2022-09-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-09
 * @FilePath: /algorithm/1598_min_operations/min_operations.go
 */

package main

func minOperations(logs []string) (depth int) {
	for _, log := range logs {
		if log == "./" {
			continue
		}
		if log != "../" {
			depth++
		} else if depth > 0 {
			depth--
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
		logs := []string{"d1/", "d2/", "../", "d21/", "./"}
		ans := 2
		assert(minOperations(logs) == ans)
	}

	{
		logs := []string{"d1/", "d2/", "./", "d3/", "../", "d31/"}
		ans := 3
		assert(minOperations(logs) == ans)
	}

	{
		logs := []string{"d1/", "../", "../", "../"}
		ans := 0
		assert(minOperations(logs) == ans)
	}
}
