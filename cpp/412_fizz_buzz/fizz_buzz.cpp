/*
 * @Date: 2021-10-13 08:44:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-13 08:51:48
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> fizzBuzz(int n) {
    vector<string> answer;
    for (int i = 1; i <= n; i++) {
      string curr;
      if (i % 3 == 0) curr += "Fizz";
      if (i % 5 == 0) curr += "Buzz";
      if (curr.size() == 0) curr += to_string(i);
      answer.emplace_back(curr);
    }
    return answer;
  }
};

int main() {
  assert(Solution().fizzBuzz(15) ==
         move(vector<string>{"1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8",
                             "Fizz", "Buzz", "11", "Fizz", "13", "14",
                             "FizzBuzz"}));
}