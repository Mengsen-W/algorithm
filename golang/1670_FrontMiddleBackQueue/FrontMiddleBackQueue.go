/*
 * @Date: 2023-11-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-28
 * @FilePath: /algorithm/golang/1670_FrontMiddleBackQueue/FrontMiddleBackQueue.go
 */

// Package main ...
package main

import (
	"container/list"
	"testing"

	"github.com/stretchr/testify/assert"
)

type FrontMiddleBackQueue struct {
	left  *list.List
	right *list.List
}

func Constructor() FrontMiddleBackQueue {
	return FrontMiddleBackQueue{
		left:  list.New(),
		right: list.New(),
	}
}

func (f *FrontMiddleBackQueue) PushFront(val int) {
	f.left.PushFront(val)
	if f.left.Len() == f.right.Len()+2 {
		f.right.PushFront(f.left.Back().Value.(int))
		f.left.Remove(f.left.Back())
	}
}

func (f *FrontMiddleBackQueue) PushMiddle(val int) {
	if f.left.Len() == f.right.Len()+1 {
		f.right.PushFront(f.left.Back().Value.(int))
		f.left.Remove(f.left.Back())
	}
	f.left.PushBack(val)
}

func (f *FrontMiddleBackQueue) PushBack(val int) {
	f.right.PushBack(val)
	if f.left.Len()+1 == f.right.Len() {
		f.left.PushBack(f.right.Front().Value.(int))
		f.right.Remove(f.right.Front())
	}
}

func (f *FrontMiddleBackQueue) PopFront() int {
	if f.left.Len() == 0 {
		return -1
	}
	val := f.left.Front().Value.(int)
	f.left.Remove(f.left.Front())
	if f.left.Len()+1 == f.right.Len() {
		f.left.PushBack(f.right.Front().Value.(int))
		f.right.Remove(f.right.Front())
	}
	return val
}

func (f *FrontMiddleBackQueue) PopMiddle() int {
	if f.left.Len() == 0 {
		return -1
	}
	val := f.left.Back().Value.(int)
	f.left.Remove(f.left.Back())
	if f.left.Len()+1 == f.right.Len() {
		f.left.PushBack(f.right.Front().Value.(int))
		f.right.Remove(f.right.Front())
	}
	return val
}

func (f *FrontMiddleBackQueue) PopBack() int {
	if f.left.Len() == 0 {
		return -1
	}
	if f.right.Len() == 0 {
		val := f.left.Back().Value.(int)
		f.left.Remove(f.left.Back())
		return val
	} else {
		val := f.right.Back().Value.(int)
		f.right.Remove(f.right.Back())
		if f.left.Len() == f.right.Len()+2 {
			f.right.PushFront(f.left.Back().Value.(int))
			f.left.Remove(f.left.Back())
		}
		return val
	}
}

func main() {
	t := &testing.T{}
	q := Constructor()

	q.PushFront(1)                    // [1]
	q.PushBack(2)                     // [1, 2]
	q.PushMiddle(3)                   // [1, 3, 2]
	q.PushMiddle(4)                   // [1, 4, 3, 2]
	assert.Equal(t, q.PopFront(), 1)  // 返回 1 -> [4, 3, 2]
	assert.Equal(t, q.PopMiddle(), 3) // 返回 3 -> [4, 2]
	assert.Equal(t, q.PopMiddle(), 4) // 返回 4 -> [2]
	assert.Equal(t, q.PopBack(), 2)   // 返回 2 -> []
	assert.Equal(t, q.PopFront(), -1) // 返回 -1 -> [] （队列为空）
}
