-- Static flag checker.

local PREFIX = "flag"
local CONTENT = "this_is_my_flag"

function check(operator_id, content)
    local ok, flag = pcall(cds.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return cds.audit.incorrect()
    end

    if flag.content == CONTENT then
        return cds.audit.correct()
    end
    return cds.audit.incorrect()
end

function generate(operator_id)
    return {}
end
