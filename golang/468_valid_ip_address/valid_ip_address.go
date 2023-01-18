/*
 * @Date: 2022-05-29 08:53:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-29 09:24:40
 * @FilePath: /algorithm/468_valid_ip_address/valid_ip_address.go
 */

package main

import (
	"reflect"
	"regexp"
)

func validIPAddress(IP string) string {
	const i8 string = `([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])`
	var v4 = regexp.MustCompile("^" + i8 + `(\.` + i8 + `){3}$`)
	const hex string = "[0-9a-f]{1,4}"
	var v6 = regexp.MustCompile("^(?i)[1-9a-f][0-9a-f]{0,3}(:" + hex + "){7}$")
	if v4.Match([]byte(IP)) {
		return "IPv4"
	}
	if v6.Match([]byte(IP)) {
		return "IPv6"
	}
	return "Neither"
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(validIPAddress("2001:0db8:85a3:0:0:8A2E:0370:7334"), "IPv6")
	assert(validIPAddress("172.16.254.1"), "IPv4")
	assert(validIPAddress("256.256.256.256"), "Neither")
	assert(validIPAddress("01.01.01.01"), "Neither")
	assert(validIPAddress("2001:db8:85a3:0::8a2E:0370:7334"), "Neither")
}
