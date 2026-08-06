-- Generic identity provider template.

function login(params)
    local auth_key = params.auth_key
    if not auth_key then
        error("missing auth_key")
    end
    return {
        auth_key = auth_key,
        username = auth_key,
        name = auth_key
    }
end

function bind(params, user)
    return login(params)
end
