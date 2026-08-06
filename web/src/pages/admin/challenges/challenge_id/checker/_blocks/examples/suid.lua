-- Dynamic UUID-like flag checker with embedded operator data.

local PREFIX = "flag"

function check(operator_id, content)
    local ok, flag = pcall(checker.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return checker.audit.incorrect()
    end

    local decoded, peer_operator_id = pcall(checker.suid.decode, flag.content)
    if not decoded then
        return checker.audit.incorrect()
    end
    if peer_operator_id ~= operator_id then
        return checker.audit.cheat(peer_operator_id)
    end
    return checker.audit.correct()
end

function generate(operator_id)
    local content = checker.suid.encode(operator_id, { hyphenated = true })
    return {
        FLAG = checker.audit.format({ prefix = PREFIX, content = content })
    }
end
