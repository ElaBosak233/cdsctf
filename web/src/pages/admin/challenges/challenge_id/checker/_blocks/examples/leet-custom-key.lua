-- Dynamic readable flag checker using a script-owned key.

local PREFIX = "flag"
local TEMPLATE = "this_is_my_flag"
local CUSTOM_KEY = "replace-with-your-own-secret"

function check(subject_id, content)
    local ok, flag = pcall(checker.audit.parse, content)
    if not ok or flag.prefix ~= PREFIX then
        return checker.audit.incorrect()
    end

    local options = { key = CUSTOM_KEY }
    local expected = checker.leet.encode(TEMPLATE, subject_id, options)
    if flag.content == expected then
        return checker.audit.correct()
    end

    local decoded, source_subject_id = pcall(
        checker.leet.decode,
        TEMPLATE,
        flag.content,
        options
    )
    if not decoded then
        return checker.audit.incorrect()
    end
    if source_subject_id ~= subject_id then
        return checker.audit.cheat(source_subject_id)
    end
    return checker.audit.incorrect()
end

function generate(subject_id)
    local content = checker.leet.encode(TEMPLATE, subject_id, {
        key = CUSTOM_KEY
    })
    return {
        FLAG = checker.audit.format({ prefix = PREFIX, content = content })
    }
end
