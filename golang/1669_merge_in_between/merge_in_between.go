/*
 * @Date: 2023-01-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-30
 * @FilePath: /algorithm/golang/1669_merge_in_between/merge_in_between.go
 */

package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeInBetween(list1 *ListNode, a int, b int, list2 *ListNode) *ListNode {
	preA := list1
	for i := 0; i < a-1; i++ {
		preA = preA.Next
	}
	preB := preA
	for i := 0; i < b-a+2; i++ {
		preB = preB.Next
	}
	preA.Next = list2
	for list2.Next != nil {
		list2 = list2.Next
	}
	list2.Next = preB
	return list1
}

func main() {
	assert := func(list1, list2 *ListNode) {
		p := list1
		q := list2
		for p != nil && q != nil && p.Val == q.Val {
			p = p.Next
			q = q.Next
		}
		if p != q {
			panic("Not Passed")
		}
	}

	{
		list1 := ListNode{Val: 0, Next: &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: &ListNode{Val: 4, Next: &ListNode{Val: 5, Next: nil}}}}}}
		list2 := ListNode{Val: 1000000, Next: &ListNode{Val: 1000001, Next: &ListNode{Val: 1000002, Next: nil}}}
		a, b := 3, 4
		ans := ListNode{Val: 0, Next: &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 1000000, Next: &ListNode{Val: 1000001, Next: &ListNode{Val: 1000002, Next: &ListNode{Val: 5, Next: nil}}}}}}}
		assert(mergeInBetween(&list1, a, b, &list2), &ans)
	}

	{
		list1 := ListNode{Val: 0, Next: &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: &ListNode{Val: 4, Next: &ListNode{Val: 5, Next: &ListNode{Val: 6, Next: nil}}}}}}}
		list2 := ListNode{Val: 1000000, Next: &ListNode{Val: 1000001, Next: &ListNode{Val: 1000002, Next: &ListNode{Val: 1000003, Next: &ListNode{Val: 1000004, Next: nil}}}}}
		a, b := 2, 5
		ans := ListNode{Val: 0, Next: &ListNode{Val: 1, Next: &ListNode{Val: 1000000, Next: &ListNode{Val: 1000001, Next: &ListNode{Val: 1000002, Next: &ListNode{Val: 1000003, Next: &ListNode{Val: 1000004, Next: &ListNode{Val: 6, Next: nil}}}}}}}}
		assert(mergeInBetween(&list1, a, b, &list2), &ans)
	}
}
