/*
 * @Date: 2022-08-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-31
 * @FilePath: /algorithm/946_validate_stack_sequences/validate_stack_sequences.go
 */

package main

func validateStackSequences(pushed, popped []int) bool {
	st := []int{}
	j := 0
	for _, x := range pushed {
		st = append(st, x)
		for len(st) > 0 && st[len(st)-1] == popped[j] {
			st = st[:len(st)-1]
			j++
		}
	}
	return len(st) == 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		pushed := []int{1, 2, 3, 4, 5}
		popped := []int{4, 5, 3, 2, 1}
		assert(validateStackSequences(pushed, popped))
	}
	{
		pushed := []int{1, 2, 3, 4, 5}
		popped := []int{4, 3, 5, 1, 2}
		assert(!validateStackSequences(pushed, popped))
	}
}
