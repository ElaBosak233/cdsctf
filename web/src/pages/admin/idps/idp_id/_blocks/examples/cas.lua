-- CAS identity provider template.

local CAS_BASE_URL = "https://cas.example.com"
local SERVICE_URL = "https://your-cdsctf.example.com/account/idps"

function login(params)
    local ticket = params.ticket
    if not ticket then
        error("missing CAS ticket")
    end

    local url = CAS_BASE_URL
        .. "/serviceValidate?format=JSON&ticket=" .. cds.http.url_encode(ticket)
        .. "&service=" .. cds.http.url_encode(SERVICE_URL)
    local response = cds.http.request(
        "GET",
        url,
        { Accept = "application/json" },
        nil
    )
    if response.status < 200 or response.status >= 300 then
        error("CAS validation request failed")
    end

    local payload = cds.json.decode(response.body)
    local success = payload.serviceResponse.authenticationSuccess
    if not success then
        error("CAS authentication failed")
    end
    local auth_key = tostring(success.user)
    local attributes = success.attributes or {}
    return {
        auth_key = auth_key,
        username = auth_key,
        name = attributes.displayName or auth_key
    }
end

function bind(params, user)
    return login(params)
end
