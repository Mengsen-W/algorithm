/*
 * @Date: 2022-11-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-06
 * @FilePath: /algorithm/1678_interpret/interpret.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string interpret(string command) {
    string res;
    for (int i = 0; i < command.size(); i++) {
      if (command[i] == 'G') {
        res += "G";
      } else if (command[i] == '(') {
        if (command[i + 1] == ')') {
          res += "o";
        } else {
          res += "al";
        }
      }
    }
    return res;
  }
};

int main() {
  assert(Solution().interpret("G()(al)") == "Goal");
  assert(Solution().interpret("G()()()()(al)") == "Gooooal");
  assert(Solution().interpret("(al)G(al)()()G") == "alGalooG");
}