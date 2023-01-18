/*
 * @Date: 2022-08-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-02
 * @FilePath: /algorithm/622_MyCircularQueue/MyCircularQueue.go
 */

package main

type MyCircularQueue struct {
	front, rear int
	elements    []int
}

func Constructor(k int) MyCircularQueue {
	return MyCircularQueue{elements: make([]int, k+1)}
}

func (q *MyCircularQueue) EnQueue(value int) bool {
	if q.IsFull() {
		return false
	}
	q.elements[q.rear] = value
	q.rear = (q.rear + 1) % len(q.elements)
	return true
}

func (q *MyCircularQueue) DeQueue() bool {
	if q.IsEmpty() {
		return false
	}
	q.front = (q.front + 1) % len(q.elements)
	return true
}

func (q MyCircularQueue) Front() int {
	if q.IsEmpty() {
		return -1
	}
	return q.elements[q.front]
}

func (q MyCircularQueue) Rear() int {
	if q.IsEmpty() {
		return -1
	}
	return q.elements[(q.rear-1+len(q.elements))%len(q.elements)]
}

func (q MyCircularQueue) IsEmpty() bool {
	return q.rear == q.front
}

func (q MyCircularQueue) IsFull() bool {
	return (q.rear+1)%len(q.elements) == q.front
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	obj := Constructor(3)
	assert(obj.EnQueue(1) == true)
	assert(obj.EnQueue(2) == true)
	assert(obj.EnQueue(3) == true)
	assert(obj.EnQueue(4) == false)
	assert(obj.Rear() == 3)
	assert(obj.IsFull() == true)
	assert(obj.DeQueue() == true)
	assert(obj.EnQueue(4) == true)
	assert(obj.Rear() == 4)
	assert(obj.Front() == 2)
}
