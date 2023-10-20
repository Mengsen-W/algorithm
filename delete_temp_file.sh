###
# @Date: 2021-08-25 12:07:19
# @Author: Mengsen Wang
# @LastEditors: 854284842@qq.com
# @LastEditTime: 2023-10-20
###

# delete single file out binary
find $(dirname "$PWD") -name "*.out" | xargs rm -rf

# delete pyc
find $(dirname "$PWD") -name "*.pyc" | xargs rm -rf

# delete tempCode
find $(dirname "$PWD") -name "tempCodeRunnerFile.*" | xargs rm -rf

# delete mac clang temp file
find $(dirname "$PWD") -name "*.dSYM" | xargs rm -rf

# delete empty dir
find $(dirname "$PWD") -type d -empty -delete
