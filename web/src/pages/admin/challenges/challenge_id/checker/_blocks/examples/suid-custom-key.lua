-- Dynamic UUID-like flag checker using a script-owned key.

local PREFIX = "flag"
local CUSTOM_KEY = "replace-with-your-own-secret"

function check(operator_id, content)
    local ok, flag = pcall(checker.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return checker.audit.incorrect()
    end

    local decoded, peer_operator_id = pcall(
        checker.suid.decode,
        flag.content,
        { key = CUSTOM_KEY }
    )
    if not decoded then
        return checker.audit.incorrect()
    end
    if peer_operator_id ~= operator_id then
        return checker.audit.cheat(peer_operator_id)
    end
    return checker.audit.correct()
end

function generate(operator_id)
    local content = checker.suid.encode(operator_id, {
        key = CUSTOM_KEY,
        hyphenated = true
    })
    return {
        FLAG = checker.audit.format({ prefix = PREFIX, content = content })
    }
end
