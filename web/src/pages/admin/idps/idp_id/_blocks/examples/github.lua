-- GitHub OAuth 2.0 authorization code flow.

local CLIENT_ID = "GITHUB_CLIENT_ID"
local CLIENT_SECRET = "GITHUB_CLIENT_SECRET"
local REDIRECT_URI = "http://localhost:5173/account/idps/1"

function login(params)
    local code = params.code
    if not code then
        error("missing authorization code")
    end

    local token_response = http.request(
        "POST",
        "https://github.com/login/oauth/access_token",
        {
            Accept = "application/json",
            ["Content-Type"] = "application/json"
        },
        json.encode({
            code = code,
            client_id = CLIENT_ID,
            client_secret = CLIENT_SECRET,
            redirect_uri = REDIRECT_URI
        })
    )
    if token_response.status < 200 or token_response.status >= 300 then
        error("GitHub token exchange failed")
    end
    local token = json.decode(token_response.body)

    local user_response = http.request(
        "GET",
        "https://api.github.com/user",
        {
            Authorization = "Bearer " .. token.access_token,
            ["User-Agent"] = "CdsCTF",
            Accept = "application/json"
        },
        nil
    )
    if user_response.status < 200 or user_response.status >= 300 then
        error("GitHub user request failed")
    end
    local user = json.decode(user_response.body)
    return {
        auth_key = tostring(user.id),
        username = user.login,
        name = user.name or user.login
    }
end

function bind(params, user)
    return login(params)
end
