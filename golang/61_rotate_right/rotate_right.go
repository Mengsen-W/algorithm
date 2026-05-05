// Package main ...
package main

import (
	"fmt"
	"reflect"
)

// ListNode Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func rotateRight(head *ListNode, k int) *ListNode {
	if k == 0 || head == nil || head.Next == nil {
		return head
	}
	n := 1
	iter := head
	for iter.Next != nil {
		iter = iter.Next
		n++
	}
	add := n - k%n
	if add == n {
		return head
	}
	iter.Next = head
	for add > 0 {
		iter = iter.Next
		add--
	}
	ret := iter.Next
	iter.Next = nil
	return ret
}

func main() {
	tests := []struct {
		head *ListNode
		k    int
		want *ListNode
	}{
		{
			&ListNode{
				1,
				&ListNode{
					2,
					&ListNode{
						3,
						&ListNode{
							4,
							&ListNode{
								5,
								nil,
							},
						},
					},
				},
			},
			2,
			&ListNode{
				4,
				&ListNode{
					5,
					&ListNode{
						1,
						&ListNode{
							2,
							&ListNode{
								3,
								nil,
							},
						},
					},
				},
			},
		},
		{
			&ListNode{0, &ListNode{1, &ListNode{2, nil}}},
			4,
			&ListNode{2, &ListNode{0, &ListNode{1, nil}}},
		},
	}

	for _, tt := range tests {
		result := rotateRight(tt.head, tt.k)
		if !reflect.DeepEqual(result, tt.want) {
			fmt.Printf("rotateRight(%v, %v) = %v, want %v\n", tt.head, tt.k, result, tt.want)
		}
	}
}
