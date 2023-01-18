/*
 * @Date: 2022-01-23 14:49:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-23 15:00:08
 * @FilePath: /algorithm/2034_stock_price/stock_price.go
 * @Description: file content
 */

package main

import (
	"container/heap"
)

type StockPrice struct {
	maxPrice, minPrice hp
	timePriceMap       map[int]int
	maxTimestamp       int
}

func Constructor() StockPrice {
	return StockPrice{timePriceMap: map[int]int{}}
}

func (sp *StockPrice) Update(timestamp, price int) {
	heap.Push(&sp.maxPrice, pair{-price, timestamp})
	heap.Push(&sp.minPrice, pair{price, timestamp})
	sp.timePriceMap[timestamp] = price
	if timestamp > sp.maxTimestamp {
		sp.maxTimestamp = timestamp
	}
}

func (sp *StockPrice) Current() int {
	return sp.timePriceMap[sp.maxTimestamp]
}

func (sp *StockPrice) Maximum() int {
	for {
		if p := sp.maxPrice[0]; -p.price == sp.timePriceMap[p.timestamp] {
			return -p.price
		}
		heap.Pop(&sp.maxPrice)
	}
}

func (sp *StockPrice) Minimum() int {
	for {
		if p := sp.minPrice[0]; p.price == sp.timePriceMap[p.timestamp] {
			return p.price
		}
		heap.Pop(&sp.minPrice)
	}
}

type pair struct{ price, timestamp int }
type hp []pair

func (h hp) Len() int            { return len(h) }
func (h hp) Less(i, j int) bool  { return h[i].price < h[j].price }
func (h hp) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{}) { *h = append(*h, v.(pair)) }
func (h *hp) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	stockPrice := Constructor()
	stockPrice.Update(1, 10)
	stockPrice.Update(2, 5)
	assert(stockPrice.Current() == 5)
	assert(stockPrice.Maximum() == 10)
	stockPrice.Update(1, 3)
	assert(stockPrice.Maximum() == 5)
	stockPrice.Update(4, 2)
	assert(stockPrice.Minimum() == 2)
}
