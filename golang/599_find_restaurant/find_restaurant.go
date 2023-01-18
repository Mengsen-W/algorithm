/*
 * @Date: 2022-03-14 00:48:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-14 01:20:21
 * @FilePath: /algorithm/599_find_restaurant/find_restaurant.go
 */

package main

import (
	"math"
	"reflect"
)

func findRestaurant(list1, list2 []string) (ans []string) {
	index := make(map[string]int, len(list1))
	for i, s := range list1 {
		index[s] = i
	}

	indexSum := math.MaxInt32
	for i, s := range list2 {
		if j, ok := index[s]; ok {
			if i+j < indexSum {
				indexSum = i + j
				ans = []string{s}
			} else if i+j == indexSum {
				ans = append(ans, s)
			}
		}
	}
	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		list1 := []string{"Shogun", "Tapioca Express", "Burger King", "KFC"}
		list2 := []string{"Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"}
		ans := []string{"Shogun"}
		assert(findRestaurant(list1, list2), ans)
	}
	{
		list1 := []string{"Shogun", "Tapioca Express", "Burger King", "KFC"}
		list2 := []string{"KFC", "Shogun", "Burger King"}
		ans := []string{"Shogun"}
		assert(findRestaurant(list1, list2), ans)
	}
}
