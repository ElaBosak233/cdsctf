export interface Config {
    site?: {
        title?: string;
        description?: string;
        color?: string;
        favicon?: string;
    };
    container?: {
        parallel_limit?: number;
        request_limit?: number;
    };
    auth?: {
        registration?: {
            enabled?: boolean;
            captcha?: boolean;
        };
    };
    captcha?: {
        provider?: string;
        turnstile?: {
            site_key?: string;
        };
        recaptcha?: {
            site_key?: string;
        };
    };
}
