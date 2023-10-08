/*
 * @Date: 2023-10-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-08
 * @FilePath: /algorithm/golang/2034_StockPrice/StockPrice.go
 */

// Package main ...
package main

import "github.com/emirpasic/gods/trees/redblacktree"

type StockPrice struct {
	prices       *redblacktree.Tree
	timePriceMap map[int]int
	maxTimestamp int
}

func Constructor() StockPrice {
	return StockPrice{redblacktree.NewWithIntComparator(), map[int]int{}, 0}
}

func (sp *StockPrice) Update(timestamp, price int) {
	if prevPrice := sp.timePriceMap[timestamp]; prevPrice > 0 {
		if times, _ := sp.prices.Get(prevPrice); times.(int) > 1 {
			sp.prices.Put(prevPrice, times.(int)-1)
		} else {
			sp.prices.Remove(prevPrice)
		}
	}
	times := 0
	if val, ok := sp.prices.Get(price); ok {
		times = val.(int)
	}
	sp.prices.Put(price, times+1)
	sp.timePriceMap[timestamp] = price
	if timestamp >= sp.maxTimestamp {
		sp.maxTimestamp = timestamp
	}
}

func (sp *StockPrice) Current() int { return sp.timePriceMap[sp.maxTimestamp] }
func (sp *StockPrice) Maximum() int { return sp.prices.Right().Key.(int) }
func (sp *StockPrice) Minimum() int { return sp.prices.Left().Key.(int) }

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	stockPrice := Constructor()
	stockPrice.Update(1, 10)           // 时间戳为 [1] ，对应的股票价格为 [10] 。
	stockPrice.Update(2, 5)            // 时间戳为 [1,2] ，对应的股票价格为 [10,5] 。
	assert(stockPrice.Current() == 5)  // 返回 5 ，最新时间戳为 2 ，对应价格为 5 。
	assert(stockPrice.Maximum() == 10) // 返回 10 ，最高价格的时间戳为 1 ，价格为 10 。
	stockPrice.Update(1, 3)            // 之前时间戳为 1 的价格错误，价格更新为 3 。
	// 时间戳为 [1,2] ，对应股票价格为 [3,5] 。
	assert(stockPrice.Maximum() == 5) // 返回 5 ，更正后最高价格为 5 。
	stockPrice.Update(4, 2)           // 时间戳为 [1,2,4] ，对应价格为 [3,5,2] 。
	assert(stockPrice.Minimum() == 2) // 返回 2 ，最低价格时间戳为 4 ，价格为 2 。
}
