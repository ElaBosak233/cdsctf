-- Dynamic UUID-like flag checker with embedded operator data.

local PREFIX = "flag"

function check(subject_id, content)
    local ok, flag = pcall(checker.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return checker.audit.incorrect()
    end

    local decoded, source_subject_id = pcall(checker.suid.decode, flag.content)
    if not decoded then
        return checker.audit.incorrect()
    end
    if source_subject_id ~= subject_id then
        return checker.audit.cheat(source_subject_id)
    end
    return checker.audit.correct()
end

function generate(subject_id)
    local content = checker.suid.encode(subject_id, { hyphenated = true })
    return {
        FLAG = checker.audit.format({ prefix = PREFIX, content = content })
    }
end
