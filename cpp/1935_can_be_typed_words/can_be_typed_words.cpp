#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int canBeTypedWords(string text, string brokenLetters) {
        unordered_set<char> broken;   // 无法输入的字符集合
        for (char ch: brokenLetters){
            broken.insert(ch);
        }
        int res = 0;   // 可以完全输入的单词数目
        bool flag = true;   // 当前字符所在单词是否可被完全输入
        for (char ch: text){
            if (ch == ' '){
                // 当前字符为空格，检查上一个单词状态，更新数目并初始化 flag
                if (flag){
                    ++res;
                }
                flag = true;
            }
            else if (broken.count(ch)){
                // 当前字符不可被输入，所在单词无法被完全输入，更新 flag
                flag = false;
            }
        }
        // 判断最后一个单词状态并更新数目
        if (flag){
            ++res;
        }
        return res;
    }
};

int main() {
  vector<tuple<string, string, int>> tests {
    {"hello world", "ad", 1},
    {"leet code", "lt", 1},
    {"leet code", "e", 0},
  };

  for (auto& [text, brokenLetters, ans] : tests) {
    assert(Solution().canBeTypedWords(text, brokenLetters) == ans);
  }
}