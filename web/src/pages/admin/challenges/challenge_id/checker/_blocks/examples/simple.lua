-- Static flag checker.

local PREFIX = "flag"
local CONTENT = "this_is_my_flag"

function check(operator_id, content)
    local ok, flag = pcall(checker.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return checker.audit.incorrect()
    end

    if flag.content == CONTENT then
        return checker.audit.correct()
    end
    return checker.audit.incorrect()
end

function generate(operator_id)
    return {}
end
