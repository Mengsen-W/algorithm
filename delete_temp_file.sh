###
 # @Date: 2021-08-25 12:07:19
 # @Author: Mengsen Wang
 # @LastEditors: 854284842@qq.com
 # @LastEditTime: 2023-01-19
###

# delete single file out binary
find $(dirname "$PWD") -name "*.out" | xargs rm -rf

# delete pyc
find $(dirname "$PWD") -name "*.pyc" | xargs rm -rf

# delete tempCode
find $(dirname "$PWD") -name "tempCodeRunnerFile.*" | xargs rm -rf

# delete mac clang temp file
find $(dirname "$PWD") -name "*.dSYM" | xargs rm -rf

deleteempty() {
    find ${1:-.} -mindepth 1 -maxdepth 1 -type d | while read -r dir
    do
        if [[ -z "$(find "$dir" -mindepth 1 -type f)" ]] >/dev/null
        then
            echo "$dir"
            rm -rf ${dir} 2>&- && echo "Empty, Deleted!" || echo "Delete error"
        fi
        if [ -d ${dir} ]
        then
            deleteempty "$dir"
        fi
    done
}

deleteempty