/*
 * @Date: 2022-05-29 08:52:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-29 09:09:55
 * @FilePath: /algorithm/468_valid_ip_address/valid_ip_address.cpp
 */

#include <cassert>
#include <regex>
#include <string>

using namespace std;

class Solution {
 public:
  string validIPAddress(const string& queryIP) {
    regex ipv4("^(?:(?:25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]?\\d)($|(?!\\.$)\\.)){4}$");
    regex ipv6("^(?:(?:[\\da-fA-F]{1,4})($|(?!:$):)){8}$");
    return regex_search(queryIP, ipv4) ? "IPv4" : regex_search(queryIP, ipv6) ? "IPv6" : "Neither";
  }
};

int main() {
  assert(Solution().validIPAddress("2001:0db8:85a3:0:0:8A2E:0370:7334") == "IPv6");
  assert(Solution().validIPAddress("172.16.254.1") == "IPv4");
  assert(Solution().validIPAddress("256.256.256.256") == "Neither");
  assert(Solution().validIPAddress("01.01.01.01") == "Neither");
  assert(Solution().validIPAddress("2001:db8:85a3:0::8a2E:0370:7334") == "Neither");
}