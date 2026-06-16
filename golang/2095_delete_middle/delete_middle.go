// Package main ...
package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteMiddle(head *ListNode) *ListNode {
	if head.Next == nil {
		return nil
	}
	slow := head
	fast := head
	var pre *ListNode

	for fast != nil && fast.Next != nil {
		fast = fast.Next.Next
		pre = slow
		slow = slow.Next
	}

	pre.Next = pre.Next.Next
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
			&ListNode{
				1,
				&ListNode{
					3,
					&ListNode{
						4,
						&ListNode{
							7,
							&ListNode{
								1,
								&ListNode{
									2,
									&ListNode{
										6, nil,
									},
								},
							},
						},
					},
				},
			},
			&ListNode{
				1,
				&ListNode{
					3,
					&ListNode{
						4,
						&ListNode{
							1,
							&ListNode{
								2,
								&ListNode{
									6, nil,
								},
							},
						},
					},
				},
			},
		},
		{
			&ListNode{
				1,
				&ListNode{
					2,
					&ListNode{
						3,
						&ListNode{
							4, nil,
						},
					},
				},
			},
			&ListNode{
				1,
				&ListNode{
					2,
					&ListNode{
						4, nil,
					},
				},
			},
		},
		{
			&ListNode{
				2,
				&ListNode{
					1, nil,
				},
			},
			&ListNode{
				2, nil,
			},
		},
	}

	for _, test := range tests {
		result := deleteMiddle(test.head)
		if !isSameList(result, test.ans) {
			fmt.Println("test failed")
		}
	}
}
