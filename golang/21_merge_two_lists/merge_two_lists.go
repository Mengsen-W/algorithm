/*
 * @Date: 2023-08-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-05
 * @FilePath: /algorithm/golang/21_merge_two_lists/merge_two_lists.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1, list2 *ListNode) *ListNode {
	if list1 == nil {
		return list2 // 注：如果都为空则返回空
	}
	if list2 == nil {
		return list1
	}
	if list1.Val < list2.Val {
		list1.Next = mergeTwoLists(list1.Next, list2)
		return list1
	}
	list2.Next = mergeTwoLists(list1, list2.Next)
	return list2
}

func main() {
	tests := []struct {
		l1  *ListNode
		l2  *ListNode
		ans *ListNode
	}{
		{
			&ListNode{1, &ListNode{2, &ListNode{4, nil}}},
			&ListNode{1, &ListNode{3, &ListNode{4, nil}}},
			&ListNode{1, &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{4, nil}}}}}},
		},
		{
			nil,
			nil,
			nil,
		},
		{
			nil,
			&ListNode{0, nil},
			&ListNode{0, nil},
		},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, mergeTwoLists(item.l1, item.l2))
	}
}
