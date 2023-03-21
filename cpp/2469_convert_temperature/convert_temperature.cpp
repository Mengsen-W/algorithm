/*
 * @Date: 2023-03-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-21
 * @FilePath: /algorithm/cpp/2469_convert_temperature/convert_temperature.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<double> convertTemperature(double celsius) { return {celsius + 273.15, celsius * 1.80 + 32.00}; }
};

int main() {
  assert((Solution().convertTemperature(36.50) == vector<double>{309.65000, 97.70000}));
  assert((Solution().convertTemperature(122.11) == vector<double>{395.26000, 251.79800}));
}
