/*
 * @Date: 2022-09-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-23
 * @FilePath: /algorithm/707_MyLinkedList/MyLinkedList.go
 */

package main

type node struct {
	val        int
	next, prev *node
}

type MyLinkedList struct {
	head, tail *node
	size       int
}

func Constructor() MyLinkedList {
	head := &node{}
	tail := &node{}
	head.next = tail
	tail.prev = head
	return MyLinkedList{head, tail, 0}
}

func (l *MyLinkedList) Get(index int) int {
	if index < 0 || index >= l.size {
		return -1
	}
	var curr *node
	if index+1 < l.size-index {
		curr = l.head
		for i := 0; i <= index; i++ {
			curr = curr.next
		}
	} else {
		curr = l.tail
		for i := 0; i < l.size-index; i++ {
			curr = curr.prev
		}
	}
	return curr.val
}

func (l *MyLinkedList) AddAtHead(val int) {
	l.AddAtIndex(0, val)
}

func (l *MyLinkedList) AddAtTail(val int) {
	l.AddAtIndex(l.size, val)
}

func (l *MyLinkedList) AddAtIndex(index, val int) {
	if index > l.size {
		return
	}
	index = max(0, index)
	var pred, succ *node
	if index < l.size-index {
		pred = l.head
		for i := 0; i < index; i++ {
			pred = pred.next
		}
		succ = pred.next
	} else {
		succ = l.tail
		for i := 0; i < l.size-index; i++ {
			succ = succ.prev
		}
		pred = succ.prev
	}
	l.size++
	toAdd := &node{val, succ, pred}
	pred.next = toAdd
	succ.prev = toAdd
}

func (l *MyLinkedList) DeleteAtIndex(index int) {
	if index < 0 || index >= l.size {
		return
	}
	var pred, succ *node
	if index < l.size-index {
		pred = l.head
		for i := 0; i < index; i++ {
			pred = pred.next
		}
		succ = pred.next.next
	} else {
		succ = l.tail
		for i := 0; i < l.size-index-1; i++ {
			succ = succ.prev
		}
		pred = succ.prev.prev
	}
	l.size--
	pred.next = succ
	succ.prev = pred
}

func max(a, b int) int {
	if b > a {
		return b
	}
	return a
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	linkedList := Constructor()
	linkedList.AddAtHead(1)
	linkedList.AddAtTail(3)
	linkedList.AddAtIndex(1, 2)    //链表变为1-> 2-> 3
	assert(linkedList.Get(1) == 2) //返回2
	linkedList.DeleteAtIndex(1)    //现在链表是1-> 3
	assert(linkedList.Get(1) == 3) //返回3
}
