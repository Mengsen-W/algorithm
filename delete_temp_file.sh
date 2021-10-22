###
 # @Date: 2021-08-25 12:07:19
 # @Author: Mengsen Wang
 # @LastEditors: Mengsen Wang
 # @LastEditTime: 2021-10-22 01:22:48
###

# delete single file out binary
find $(dirname "$PWD") -name "*.out" | xargs rm -rf

# delete pyc
find $(dirname "$PWD") -name "*.pyc" | xargs rm -rf

# delete tempCode
find $(dirname "$PWD") -name "tempCodeRunnerFile.*" | xargs rm -rf
