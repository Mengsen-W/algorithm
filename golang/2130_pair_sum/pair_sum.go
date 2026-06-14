// Package main ...
package main

import "fmt"

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func pairSum(head *ListNode) int {
	slow := head
	fast := head.Next
	for fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}

	// 反转后半部分链表
	last := slow.Next
	for last.Next != nil {
		cur := last.Next
		last.Next = cur.Next
		cur.Next = slow.Next
		slow.Next = cur
	}

	ans := 0
	x := head
	y := slow.Next
	for y != nil {
		if x.Val+y.Val > ans {
			ans = x.Val + y.Val
		}
		x = x.Next
		y = y.Next
	}
	return ans
}

func main() {
	tests := []struct {
		head *ListNode
		ans  int
	}{
		{&ListNode{5, &ListNode{4, &ListNode{2, &ListNode{1, nil}}}}, 6},
		{&ListNode{4, &ListNode{2, &ListNode{2, &ListNode{3, nil}}}}, 7},
		{&ListNode{1, &ListNode{100000, nil}}, 100001},
	}

	for _, test := range tests {
		if pairSum(test.head) != test.ans {
			fmt.Println("test failed")
		}
	}
}
