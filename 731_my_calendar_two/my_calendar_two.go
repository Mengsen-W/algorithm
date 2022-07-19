/*
 * @Date: 2022-07-19
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-19
 * @FilePath: /algorithm/731_my_calendar_two/my_calendar_two.go
 */

package main

type pair struct{ first, second int }
type MyCalendarTwo map[int]pair

func Constructor() MyCalendarTwo {
	return MyCalendarTwo{}
}

func (tree MyCalendarTwo) update(start, end, val, l, r, idx int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	if r < start || end < l {
		return
	}
	if start <= l && r <= end {
		p := tree[idx]
		p.first += val
		p.second += val
		tree[idx] = p
		return
	}
	mid := (l + r) >> 1
	tree.update(start, end, val, l, mid, 2*idx)
	tree.update(start, end, val, mid+1, r, 2*idx+1)
	p := tree[idx]
	p.first = p.second + max(tree[2*idx].first, tree[2*idx+1].first)
	tree[idx] = p
}

func (tree MyCalendarTwo) Book(start, end int) bool {
	tree.update(start, end-1, 1, 0, 1e9, 1)
	if tree[1].first > 2 {
		tree.update(start, end-1, -1, 0, 1e9, 1)
		return false
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	m := Constructor()
	assert(m.Book(10, 20)) // returns true
	assert(m.Book(50, 60)) // returns true
	assert(m.Book(10, 40)) // returns true
	assert(!m.Book(5, 15)) // returns false
	assert(m.Book(5, 10))  // returns true
	assert(m.Book(25, 55)) // returns true
}
