/*
 * @Date: 2021-08-27 18:15:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-27 18:28:43
 */

package main

import "github.com/emirpasic/gods/trees/redblacktree"

type MedianFinder struct {
	nums        *redblacktree.Tree
	total       int
	left, right iterator
}

func Constructor() MedianFinder {
	return MedianFinder{nums: redblacktree.NewWithIntComparator()}
}

func (mf *MedianFinder) AddNum(num int) {
	if count, has := mf.nums.Get(num); has {
		mf.nums.Put(num, count.(int)+1)
	} else {
		mf.nums.Put(num, 1)
	}
	if mf.total == 0 {
		it := mf.nums.Iterator()
		it.Next()
		mf.left = iterator{it, 1}
		mf.right = mf.left
	} else if mf.total%2 == 1 {
		if num < mf.left.Key().(int) {
			mf.left.prev()
		} else {
			mf.right.next()
		}
	} else {
		if mf.left.Key().(int) < num && num < mf.right.Key().(int) {
			mf.left.next()
			mf.right.prev()
		} else if num >= mf.right.Key().(int) {
			mf.left.next()
		} else {
			mf.right.prev()
			mf.left = mf.right
		}
	}
	mf.total++
}

func (mf *MedianFinder) FindMedian() float64 {
	return float64(mf.left.Key().(int)+mf.right.Key().(int)) / 2
}

type iterator struct {
	redblacktree.Iterator
	count int
}

func (it *iterator) prev() {
	if it.count > 1 {
		it.count--
	} else {
		it.Prev()
		it.count = it.Value().(int)
	}
}

func (it *iterator) next() {
	if it.count < it.Value().(int) {
		it.count++
	} else {
		it.Next()
		it.count = 1
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	obj := &MedianFinder()
	obj.addNum(1)
	obj.addNum(2)
	assert(obj.findMedian() == 1.5)
	obj.addNum(3)
	assert(obj.findMedian() == 2)
}
