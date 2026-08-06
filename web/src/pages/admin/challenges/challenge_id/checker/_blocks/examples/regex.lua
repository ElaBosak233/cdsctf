-- Regular expression flag checker.

local PREFIX = "flag"
local PATTERN = "^this_is_my_flag_[0-9]+$"

function check(operator_id, content)
    local ok, flag = pcall(checker.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return checker.audit.incorrect()
    end

    if regex.is_match(PATTERN, flag.content) then
        return checker.audit.correct()
    end
    return checker.audit.incorrect()
end

function generate(operator_id)
    return {}
end
