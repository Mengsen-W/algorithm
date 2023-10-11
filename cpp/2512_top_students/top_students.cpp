/*
 * @Date: 2023-10-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-11
 * @FilePath: /algorithm/cpp/2512_top_students/top_students.cpp
 */

#include <cassert>
#include <sstream>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> topStudents(vector<string>& positive_feedback, vector<string>& negative_feedback, vector<string>& report,
                          vector<int>& student_id, int k) {
    unordered_map<std::string, int> words;
    for (const auto& word : positive_feedback) {
      words[word] = 3;
    }
    for (const auto& word : negative_feedback) {
      words[word] = -1;
    }
    vector<vector<int>> A;
    for (int i = 0; i < report.size(); i++) {
      stringstream ss;  // stream根据空格分词
      string w;
      int score = 0;
      ss << report[i];
      while (ss >> w) {
        if (words.count(w)) {
          score += words[w];
        }
      }
      A.push_back({-score, student_id[i]});
    }
    sort(A.begin(), A.end());
    vector<int> top_k;
    for (int i = 0; i < k; i++) {
      top_k.push_back(A[i][1]);
    }
    return top_k;
  }
};

int main() {
  vector<tuple<vector<string>, vector<string>, vector<string>, vector<int>, int, vector<int>>> tests{
      {
          {"smart", "brilliant", "studious"},
          {"not"},
          {"this student is studious", "the student is smart"},
          {1, 2},
          2,
          {1, 2},
      },
      {
          {"smart", "brilliant", "studious"},
          {"not"},
          {"this student is not studious", "the student is smart"},
          {1, 2},
          2,
          {2, 1},
      },
  };

  for (auto& [positive_feedback, negative_feedback, report, student_id, k, ans] : tests) {
    assert(Solution().topStudents(positive_feedback, negative_feedback, report, student_id, k) == ans);
  }
}
