/*
 * @Date: 2023-07-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-02
 * @FilePath: /algorithm/golang/2_add_two_numbers/add_two_numbers.go
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

func addTwoNumbers(l1, l2 *ListNode) (head *ListNode) {
	var tail *ListNode
	carry := 0
	for l1 != nil || l2 != nil {
		n1, n2 := 0, 0
		if l1 != nil {
			n1 = l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			n2 = l2.Val
			l2 = l2.Next
		}
		sum := n1 + n2 + carry
		sum, carry = sum%10, sum/10
		if head == nil {
			head = &ListNode{Val: sum}
			tail = head
		} else {
			tail.Next = &ListNode{Val: sum}
			tail = tail.Next
		}
	}
	if carry > 0 {
		tail.Next = &ListNode{Val: carry}
	}
	return
}

func isSame(l1, l2 *ListNode) bool {
	if l1 == nil && l2 == nil {
		return true
	}
	if l1 != nil && l2 != nil {
		return l1.Val == l2.Val && isSame(l1.Next, l2.Next)
	}
	return false
}

func main() {
	testMap := []struct {
		l1  *ListNode
		l2  *ListNode
		ans *ListNode
	}{
		{
			&ListNode{2, &ListNode{4, &ListNode{3, nil}}},
			&ListNode{5, &ListNode{6, &ListNode{4, nil}}},
			&ListNode{7, &ListNode{0, &ListNode{8, nil}}},
		},
		{
			&ListNode{0, nil},
			&ListNode{0, nil},
			&ListNode{0, nil},
		},
		{
			&ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, nil}}}}}}},
			&ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, nil}}}},
			&ListNode{8, &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{0, &ListNode{0, &ListNode{0, &ListNode{1, nil}}}}}}}},
		},
	}

	for _, item := range testMap {
		assert.True(&testing.T{}, isSame(addTwoNumbers(item.l1, item.l2), item.ans))
	}
}
