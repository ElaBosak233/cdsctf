-- Dynamic UUID-like flag checker with embedded operator data.

local PREFIX = "flag"

function check(operator_id, content)
    local ok, flag = pcall(cds.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return cds.audit.incorrect()
    end

    local decoded, peer_operator_id = pcall(cds.suid.decode, flag.content, cds.fs.key())
    if not decoded then
        return cds.audit.incorrect()
    end
    if peer_operator_id ~= operator_id then
        return cds.audit.cheat(peer_operator_id)
    end
    return cds.audit.correct()
end

function generate(operator_id)
    local content = cds.suid.encode(operator_id, cds.fs.key(), true)
    return {
        FLAG = cds.audit.format({ prefix = PREFIX, content = content })
    }
end
