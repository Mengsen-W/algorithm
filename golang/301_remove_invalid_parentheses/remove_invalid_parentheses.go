/*
 * @Date: 2021-10-27 01:50:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-27 02:37:20
 * @FilePath: /algorithm/301_remove_invalid_parentheses/remove_invalid_parentheses.go
 * @Description: file content
 */

package main

import (
	"math/bits"
	"reflect"
)

func checkValid(str string, lmask, rmask int, left, right []int) bool {
	cnt := 0
	pos1, pos2 := 0, 0
	for i := range str {
		if pos1 < len(left) && i == left[pos1] {
			if lmask>>pos1&1 == 0 {
				cnt++
			}
			pos1++
		} else if pos2 < len(right) && i == right[pos2] {
			if rmask>>pos2&1 == 0 {
				cnt--
				if cnt < 0 {
					return false
				}
			}
			pos2++
		}
	}
	return cnt == 0
}

func recoverStr(str string, lmask, rmask int, left, right []int) string {
	res := []rune{}
	pos1, pos2 := 0, 0
	for i, ch := range str {
		if pos1 < len(left) && i == left[pos1] {
			if lmask>>pos1&1 == 0 {
				res = append(res, ch)
			}
			pos1++
		} else if pos2 < len(right) && i == right[pos2] {
			if rmask>>pos2&1 == 0 {
				res = append(res, ch)
			}
			pos2++
		} else {
			res = append(res, ch)
		}
	}
	return string(res)
}

func removeInvalidParentheses(s string) (ans []string) {
	var left, right []int
	lremove, rremove := 0, 0
	for i, ch := range s {
		if ch == '(' {
			left = append(left, i)
			lremove++
		} else if ch == ')' {
			right = append(right, i)
			if lremove == 0 {
				rremove++
			} else {
				lremove--
			}
		}
	}

	var maskArr1, maskArr2 []int
	for i := 0; i < 1<<len(left); i++ {
		if bits.OnesCount(uint(i)) == lremove {
			maskArr1 = append(maskArr1, i)
		}
	}
	for i := 0; i < 1<<len(right); i++ {
		if bits.OnesCount(uint(i)) == rremove {
			maskArr2 = append(maskArr2, i)
		}
	}

	res := map[string]struct{}{}
	for _, mask1 := range maskArr1 {
		for _, mask2 := range maskArr2 {
			if checkValid(s, mask1, mask2, left, right) {
				res[recoverStr(s, mask1, mask2, left, right)] = struct{}{}
			}
		}
	}
	for str := range res {
		ans = append(ans, str)
	}
	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		s := "()())()"
		ans := []string{"(())()", "()()()"}
		assert(removeInvalidParentheses(s), ans)
	}
	{
		s := "(a)())()"
		ans := []string{"(a())()", "(a)()()"}
		assert(removeInvalidParentheses(s), ans)
	}
	{
		s := ")("
		ans := []string{""}
		assert(removeInvalidParentheses(s), ans)
	}
}
