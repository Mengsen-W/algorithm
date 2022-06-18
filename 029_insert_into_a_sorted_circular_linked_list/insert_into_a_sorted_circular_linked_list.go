/*
 * @Date: 2022-06-18 10:04:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-18 10:31:57
 * @FilePath: /algorithm/029_insert_into_a_sorted_circular_linked_list/insert_into_a_sorted_circular_linked_list.go
 */

package main

// Definition for a Node.
type Node struct {
	Val  int
	Next *Node
}

func insert(head *Node, insertVal int) *Node {
	node := &Node{Val: insertVal}
	if head == nil {
		node.Next = node
		return node
	}
	if head.Next == head {
		head.Next = node
		node.Next = head
		return head
	}
	curr, next := head, head.Next
	for next != head {
		if insertVal >= curr.Val && insertVal <= next.Val {
			break
		}
		if curr.Val > next.Val {
			if insertVal > curr.Val || insertVal < next.Val {
				break
			}
		}
		curr = curr.Next
		next = next.Next
	}
	curr.Next = node
	node.Next = next
	return head
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		head := &Node{3, &Node{4, &Node{1, nil}}}
		assert(insert(head, 2).Val == 3)
	}

	{
		head := &Node{}
		assert(insert(head, 1).Val == 1)
	}

	{
		head := &Node{1, nil}
		assert(insert(head, 2).Val == 1)
	}
}
