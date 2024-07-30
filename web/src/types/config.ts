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
    user?: {
        register?: {
            enabled?: boolean;
            captcha?: {
                enabled?: boolean;
            };
            email?: {
                domains?: Array<string>;
                enabled?: boolean;
            };
        };
    };
}
