-- Dynamic readable flag checker with embedded operator data.

local PREFIX = "flag"
local TEMPLATE = "this_is_my_flag"

function check(operator_id, content)
    local ok, flag = pcall(cds.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return cds.audit.incorrect()
    end

    local key = cds.fs.key()
    local expected = cds.leet.encode(TEMPLATE, operator_id, key)
    if flag.content == expected then
        return cds.audit.correct()
    end

    local decoded, peer_operator_id = pcall(cds.leet.decode, TEMPLATE, flag.content, key)
    if not decoded then
        return cds.audit.incorrect()
    end
    if peer_operator_id ~= operator_id then
        return cds.audit.cheat(peer_operator_id)
    end
    return cds.audit.incorrect()
end

function generate(operator_id)
    local content = cds.leet.encode(TEMPLATE, operator_id, cds.fs.key())
    return {
        FLAG = cds.audit.format({ prefix = PREFIX, content = content })
    }
end
