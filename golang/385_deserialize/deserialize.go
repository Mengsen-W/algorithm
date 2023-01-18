/*
 * @Date: 2022-04-15 09:32:18
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-19
 * @FilePath: /golang/385_deserialize/deserialize.go
 */

package main

import (
	"strconv"
	"unicode"
)

type NestedInteger struct {
}

// Return true if this NestedInteger holds a single integer, rather than a nested list.
func (n NestedInteger) IsInteger() bool { return true }

// Return the single integer that this NestedInteger holds, if it holds a single integer
// The result is undefined if this NestedInteger holds a nested list
// So before calling this method, you should have a check
func (n NestedInteger) GetInteger() int { return 0 }

// Set this NestedInteger to hold a single integer.
func (n *NestedInteger) SetInteger(value int) { return }

// Set this NestedInteger to hold a nested list and adds a nested integer to it.
func (n *NestedInteger) Add(elem NestedInteger) { return }

// Return the nested list that this NestedInteger holds, if it holds a nested list
// The list length is zero if this NestedInteger holds a single integer
// You can access NestedInteger's List element directly if you want to modify it
func (n NestedInteger) GetList() []*NestedInteger { return []*NestedInteger{} }

func deserialize(s string) *NestedInteger {
	if s[0] != '[' {
		num, _ := strconv.Atoi(s)
		ni := &NestedInteger{}
		ni.SetInteger(num)
		return ni
	}
	stack, num, negative := []*NestedInteger{}, 0, false
	for i, ch := range s {
		if ch == '-' {
			negative = true
		} else if unicode.IsDigit(ch) {
			num = num*10 + int(ch-'0')
		} else if ch == '[' {
			stack = append(stack, &NestedInteger{})
		} else if ch == ',' || ch == ']' {
			if unicode.IsDigit(rune(s[i-1])) {
				if negative {
					num = -num
				}
				ni := NestedInteger{}
				ni.SetInteger(num)
				stack[len(stack)-1].Add(ni)
			}
			num, negative = 0, false
			if ch == ']' && len(stack) > 1 {
				stack[len(stack)-2].Add(*stack[len(stack)-1])
				stack = stack[:len(stack)-1]
			}
		}
	}
	return stack[len(stack)-1]
}
