/*
 * @Date: 2021-12-26 01:06:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-26 01:47:50
 */

#include <cassert>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> findOcurrences(string text, string first, string second) {
    istringstream record(text);
    string one, two, three;
    record >> one;
    record >> two;
    vector<string> ans;
    while (record >> three) {
      if (one == first && two == second) ans.emplace_back(three);
      one = two;
      two = three;
    }
    return ans;
  }
};

int main() {
  {
    string text = "alice is a good girl she is a good student";
    string first = "a";
    string second = "good";
    assert((Solution().findOcurrences(text, first, second) ==
            vector<string>{"girl", "student"}));
  }

  {
    string text = "we will we will rock you";
    string first = "we";
    string second = "will";
    assert((Solution().findOcurrences(text, first, second) ==
            vector<string>{"we", "rock"}));
  }
}