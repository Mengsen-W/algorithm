/*
 * @Date: 2022-04-10 09:53:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-10 10:41:20
 * @FilePath: /algorithm/804_unique_morse_representations/unique_morse_representations.cpp
 * /algorithm/804_unique_morse_representations/unique_morse_representations.cpp
 * /algorithm/804_unique_morse_representations/unique_morse_representations.cpp
 */

#include <cassert>
#include <string>
#include <unordered_set>
#include <vector>

using namespace std;

const static string MORSE[] = {
    ".-",   "-...", "-.-.", "-..",  ".",   "..-.", "--.",  "....", "..",
    ".---", "-.-",  ".-..", "--",   "-.",  "---",  ".--.", "--.-", ".-.",
    "...",  "-",    "..-",  "...-", ".--", "-..-", "-.--", "--.."};

class Solution {
 public:
  int uniqueMorseRepresentations(vector<string> words) {
    unordered_set<string> seen;
    for (auto &word : words) {
      string code;
      for (auto &c : word) {
        code.append(MORSE[c - 'a']);
      }
      seen.emplace(code);
    }
    return seen.size();
  }
};

int main() {
  assert(Solution().uniqueMorseRepresentations(
             vector<string>{"gin", "zen", "gig", "msg"}) == 2);
  assert(Solution().uniqueMorseRepresentations(vector<string>{"a"}) == 1);
}