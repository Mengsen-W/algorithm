/*
 * @Date: 2023-04-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-10
 * @FilePath: /algorithm/golang/1019_next_larger_nodes/next_larger_nodes.go
 */

// Package main ...
package main

import "reflect"

// ListNode Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func nextLargerNodes(head *ListNode) []int {
	var ans []int
	var stack [][]int
	cur := head
	idx := -1
	for cur != nil {
		idx++
		ans = append(ans, 0)
		for len(stack) > 0 && stack[len(stack)-1][0] < cur.Val {
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			ans[top[1]] = cur.Val
		}
		stack = append(stack, []int{cur.Val, idx})
		cur = cur.Next
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		head := &ListNode{2, &ListNode{1, &ListNode{5, nil}}}
		ans := []int{5, 5, 0}
		assert(nextLargerNodes(head), ans)
	}

	{
		head := &ListNode{2, &ListNode{7, &ListNode{4, &ListNode{3, &ListNode{5, nil}}}}}
		ans := []int{7, 0, 5, 5, 0}
		assert(nextLargerNodes(head), ans)
	}
}
