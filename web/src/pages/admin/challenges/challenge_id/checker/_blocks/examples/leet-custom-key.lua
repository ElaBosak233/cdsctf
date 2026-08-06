-- Dynamic readable flag checker using a script-owned key.

local PREFIX = "flag"
local TEMPLATE = "this_is_my_flag"
local CUSTOM_KEY = "replace-with-your-own-secret"

function check(operator_id, content)
    local ok, flag = pcall(checker.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return checker.audit.incorrect()
    end

    local options = { key = CUSTOM_KEY }
    local expected = checker.leet.encode(TEMPLATE, operator_id, options)
    if flag.content == expected then
        return checker.audit.correct()
    end

    local decoded, peer_operator_id = pcall(
        checker.leet.decode,
        TEMPLATE,
        flag.content,
        options
    )
    if not decoded then
        return checker.audit.incorrect()
    end
    if peer_operator_id ~= operator_id then
        return checker.audit.cheat(peer_operator_id)
    end
    return checker.audit.incorrect()
end

function generate(operator_id)
    local content = checker.leet.encode(TEMPLATE, operator_id, {
        key = CUSTOM_KEY
    })
    return {
        FLAG = checker.audit.format({ prefix = PREFIX, content = content })
    }
end
