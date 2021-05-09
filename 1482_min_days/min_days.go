/*
 * @Date: 2021-05-09 08:52:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-09 10:12:26
 */

package main

import (
	"fmt"
	"sort"
)

func minDays(bloomDay []int, m, k int) int {
	if k*m > len(bloomDay) {
		return -1
	}
	maxDay := 0
	for _, day := range bloomDay {
		if day > maxDay {
			maxDay = day
		}
	}
	return sort.Search(maxDay, func(days int) bool {
		flowers, bouquets := 0, 0
		for _, d := range bloomDay {
			if d > days {
				flowers = 0
			} else {
				flowers++
				if flowers == k {
					bouquets++
					flowers = 0
				}
			}
		}
		return bouquets >= m
	})
}

func main() {
	{
		bloomDay := []int{1, 10, 3, 10, 2}
		if minDays(bloomDay, 3, 1) != 3 {
			fmt.Println("Not Passed!")
		}
	}
	{
		bloomDay := []int{1, 10, 3, 10, 2}
		if minDays(bloomDay, 3, 2) != -1 {
			fmt.Println("Not Passed!")
		}
	}
	{
		bloomDay := []int{7, 7, 7, 7, 12, 7, 7}
		if minDays(bloomDay, 2, 3) != 12 {
			fmt.Println("Not Passed!")
		}
	}
	{
		bloomDay := []int{1000000000, 1000000000}
		if minDays(bloomDay, 1, 1) != 1000000000 {
			fmt.Println("Not Passed!")
		}
	}
	{
		bloomDay := []int{1, 10, 2, 9, 3, 8, 4, 7, 5, 6}
		if minDays(bloomDay, 4, 2) != 9 {
			fmt.Println("Not Passed!")
		}
	}
}
