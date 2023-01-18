/*
 * @Date: 2022-12-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-23
 * @FilePath: /algorithm/2011_final_value_after_operations/final_value_after_operations.go
 */

package main

func finalValueAfterOperations(operations []string) (x int) {
	for _, op := range operations {
		if op[1] == '+' {
			x++
		} else {
			x--
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
		operations := []string{"--X", "X++", "X++"}
		ans := 1
		assert(finalValueAfterOperations(operations) == ans)
	}

	{
		operations := []string{"++X", "++X", "X++"}
		ans := 3
		assert(finalValueAfterOperations(operations) == ans)
	}

	{
		operations := []string{"X++", "++X", "--X", "X--"}
		ans := 0
		assert(finalValueAfterOperations(operations) == ans)
	}
}
