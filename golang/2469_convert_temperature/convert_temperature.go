/*
 * @Date: 2023-03-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-21
 * @FilePath: /algorithm/golang/2469_convert_temperature/convert_temperature.go
 */

// Package main ...
package main

import "reflect"

func convertTemperature(celsius float64) []float64 {
	return []float64{celsius + 273.15, celsius*1.80 + 32.00}
}

func main() {
	assert := func(a, b []float64) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(convertTemperature(36.50), []float64{309.65000, 97.70000})
	assert(convertTemperature(122.11), []float64{395.26000, 251.79800})
}
