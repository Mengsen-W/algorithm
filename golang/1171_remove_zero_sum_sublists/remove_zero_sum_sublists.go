/*
 * @Date: 2023-06-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-11
 * @FilePath: /algorithm/golang/1171_remove_zero_sum_sublists/remove_zero_sum_sublists.go
 */

// Package main ...
package main

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func removeZeroSumSublists(head *ListNode) *ListNode {
	dummy := &ListNode{Val: 0}
	dummy.Next = head
	seen := map[int]*ListNode{}
	prefix := 0
	for node := dummy; node != nil; node = node.Next {
		prefix += node.Val
		seen[prefix] = node
	}
	prefix = 0
	for node := dummy; node != nil; node = node.Next {
		prefix += node.Val
		node.Next = seen[prefix].Next
	}
	return dummy.Next
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	var listNodeToString func(head *ListNode) string
	listNodeToString = func(head *ListNode) string {
		result := "["
		if head != nil {
			result += string(head.Val)
			result += listNodeToString(head.Next)
		}
		return result + "]"
	}

	{
		header := &ListNode{1, &ListNode{2, &ListNode{-3, &ListNode{3, &ListNode{1, nil}}}}}
		ans := &ListNode{3, &ListNode{1, nil}}
		assert(listNodeToString(removeZeroSumSublists(header)) == listNodeToString(ans))
	}

	{
		header := &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{-3, &ListNode{4, nil}}}}}
		ans := &ListNode{1, &ListNode{2, &ListNode{4, nil}}}
		assert(listNodeToString(removeZeroSumSublists(header)) == listNodeToString(ans))
	}

	{
		header := &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{-3, &ListNode{-2, nil}}}}}
		ans := &ListNode{1, nil}
		assert(listNodeToString(removeZeroSumSublists(header)) == listNodeToString(ans))
	}
}
