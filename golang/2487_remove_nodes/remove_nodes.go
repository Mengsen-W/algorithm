/*
 * @Date: 2024-01-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-03
 * @FilePath: /algorithm/golang/2487_remove_nodes/remove_nodes.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// ListNode Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func reverse(head *ListNode) *ListNode {
	dummy := &ListNode{}
	for head != nil {
		p := head
		head = head.Next
		p.Next = dummy.Next
		dummy.Next = p
	}
	return dummy.Next
}

func removeNodes(head *ListNode) *ListNode {
	head = reverse(head)
	for p := head; p.Next != nil; {
		if p.Val > p.Next.Val {
			p.Next = p.Next.Next
		} else {
			p = p.Next
		}
	}
	return reverse(head)
}

func isSameList(l1, l2 *ListNode) bool {
	for l1 != nil && l2 != nil {
		if l1.Val != l2.Val {
			return false
		}
		l1 = l1.Next
		l2 = l2.Next
	}
	if l1 != nil || l2 != nil {
		return false
	}
	return true
}

func main() {
	tests := []struct {
		head *ListNode
		ans  *ListNode
	}{
		{
			&ListNode{5, &ListNode{2, &ListNode{13, &ListNode{3, &ListNode{8, nil}}}}},
			&ListNode{13, &ListNode{8, nil}},
		},
		{
			&ListNode{1, &ListNode{1, &ListNode{1, &ListNode{1, nil}}}},
			&ListNode{1, &ListNode{1, &ListNode{1, &ListNode{1, nil}}}},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, true, isSameList(removeNodes(test.head), test.ans), index)
	}
}
