/*
 * @Date: 2022-08-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-01
 * @FilePath: /algorithm/1374_generate_the_string/generate_the_string.cpp
 */

#include <iostream>
#include <string>

using namespace std;

class Solution {
 public:
  string generateTheString(int n) {
    if (n % 2 == 1) {
      return string(n, 'a');
    }
    return string(n - 1, 'a') + 'b';
  }
};

int main() {
  std::cout << Solution().generateTheString(4) << std::endl;
  std::cout << Solution().generateTheString(2) << std::endl;
  return 0;
}