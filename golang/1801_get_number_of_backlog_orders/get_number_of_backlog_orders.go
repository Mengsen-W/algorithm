/*
 * @Date: 2023-01-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2023-01-02
 * @FilePath: /algorithm/1801_get_number_of_backlog_orders/get_number_of_backlog_orders.go
 */

package main

import "container/heap"

func getNumberOfBacklogOrders(orders [][]int) (ans int) {
	buyOrders, sellOrders := hp{}, hp2{}
	for _, o := range orders {
		price, amount := o[0], o[1]
		if o[2] == 0 {
			for amount > 0 && len(sellOrders) > 0 && sellOrders[0].price <= price {
				if sellOrders[0].left > amount {
					sellOrders[0].left -= amount
					amount = 0
					break
				}
				amount -= heap.Pop(&sellOrders).(pair).left
			}
			if amount > 0 {
				heap.Push(&buyOrders, pair{price, amount})
			}
		} else {
			for amount > 0 && len(buyOrders) > 0 && buyOrders[0].price >= price {
				if buyOrders[0].left > amount {
					buyOrders[0].left -= amount
					amount = 0
					break
				}
				amount -= heap.Pop(&buyOrders).(pair).left
			}
			if amount > 0 {
				heap.Push(&sellOrders, pair{price, amount})
			}
		}
	}
	for _, p := range buyOrders {
		ans += p.left
	}
	for _, p := range sellOrders {
		ans += p.left
	}
	return ans % (1e9 + 7)
}

type pair struct{ price, left int }
type hp []pair

func (h hp) Len() int            { return len(h) }
func (h hp) Less(i, j int) bool  { return h[i].price > h[j].price }
func (h hp) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{}) { *h = append(*h, v.(pair)) }
func (h *hp) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }

type hp2 []pair

func (h hp2) Len() int            { return len(h) }
func (h hp2) Less(i, j int) bool  { return h[i].price < h[j].price }
func (h hp2) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *hp2) Push(v interface{}) { *h = append(*h, v.(pair)) }
func (h *hp2) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		orders := [][]int{{10, 5, 0}, {15, 2, 1}, {25, 1, 1}, {30, 4, 0}}
		ans := 6
		assert(getNumberOfBacklogOrders(orders) == ans)
	}

	{
		orders := [][]int{{7, 1000000000, 1}, {15, 3, 0}, {5, 999999995, 0}, {5, 1, 1}}
		ans := 999999984
		assert(getNumberOfBacklogOrders(orders) == ans)
	}
}
