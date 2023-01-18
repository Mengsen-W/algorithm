/*
 * @Date: 2021-06-05 09:12:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-05 10:10:38
 */

package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeElements_recursive(head *ListNode, val int) *ListNode {
	if head == nil {
		return head
	}
	head.Next = removeElements_recursive(head.Next, val)
	if head.Val == val {
		return head.Next
	}
	return head
}

func removeElements_iteration(head *ListNode, val int) *ListNode {
	dummyHead := &ListNode{Next: head}
	for tmp := dummyHead; tmp.Next != nil; {
		if tmp.Next.Val == val {
			tmp.Next = tmp.Next.Next
		} else {
			tmp = tmp.Next
		}
	}
	return dummyHead.Next
}

func vec_to_list(nums []int) *ListNode {
	size := len(nums)
	if size == 0 {
		return nil
	}
	var res, cur *ListNode
	res = &ListNode{}
	cur = res

	for i := 0; i < size; i++ {
		cur.Val = nums[i]
		if i == size-1 {
			cur.Next = nil
		} else {
			cur.Next = &ListNode{}
		}
		cur = cur.Next
	}

	return res
}

func print_list(list *ListNode) {
	for list != nil {
		fmt.Printf("%d", list.Val)
		list = list.Next
	}
	fmt.Println("")
}

func main() {
	print_list(removeElements_iteration(vec_to_list([]int{1, 2, 6, 3, 4, 5, 6}), 6))
	print_list(removeElements_recursive(vec_to_list([]int{1, 2, 6, 3, 4, 5, 6}), 6))
	print_list(removeElements_iteration(vec_to_list([]int{}), 1))
	print_list(removeElements_recursive(vec_to_list([]int{}), 1))
	print_list(removeElements_iteration(vec_to_list([]int{7, 7, 7, 7, 7}), 7))
	print_list(removeElements_recursive(vec_to_list([]int{7, 7, 7, 7, 7}), 7))
}
