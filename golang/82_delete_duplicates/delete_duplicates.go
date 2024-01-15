/*
 * @Date: 2024-01-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-15
 * @FilePath: /algorithm/golang/82_delete_duplicates/delete_duplicates.go
 */

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

func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	dummy := &ListNode{0, head}

	cur := dummy
	for cur.Next != nil && cur.Next.Next != nil {
		if cur.Next.Val == cur.Next.Next.Val {
			x := cur.Next.Val
			for cur.Next != nil && cur.Next.Val == x {
				cur.Next = cur.Next.Next
			}
		} else {
			cur = cur.Next
		}
	}

	return dummy.Next
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
			&ListNode{
				1, &ListNode{2, &ListNode{3, &ListNode{3, &ListNode{4, &ListNode{4, &ListNode{5, nil}}}}}},
			},
			&ListNode{1, &ListNode{2, &ListNode{5, nil}}},
		},
		{
			&ListNode{1, &ListNode{1, &ListNode{1, &ListNode{2, &ListNode{3, nil}}}}},
			&ListNode{2, &ListNode{3, nil}},
		},
	}

	for index, test := range tests {
		assert.True(&testing.T{}, isSameList(deleteDuplicates(test.head), test.ans), index)
	}
}
