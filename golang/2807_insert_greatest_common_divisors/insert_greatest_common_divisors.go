/*
 * @Date: 2024-01-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-06
 * @FilePath: /algorithm/golang/2807_insert_greatest_common_divisors/insert_greatest_common_divisors.go
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

func insertGreatestCommonDivisors(head *ListNode) *ListNode {
	gcd := func(a, b int) int {
		for b != 0 {
			a, b = b, a%b
		}
		return a
	}
	node := head
	for node.Next != nil {
		node.Next = &ListNode{gcd(node.Val, node.Next.Val), node.Next}
		node = node.Next.Next
	}
	return head
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
			&ListNode{18, &ListNode{6, &ListNode{10, &ListNode{3, nil}}}},
			&ListNode{18, &ListNode{6, &ListNode{6, &ListNode{2, &ListNode{10, &ListNode{1, &ListNode{3, nil}}}}}}},
		},
		{&ListNode{7, nil}, &ListNode{7, nil}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, true, isSameList(test.ans, insertGreatestCommonDivisors(test.head)), index)
	}
}
