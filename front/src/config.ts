function get_api_prefix() {
    if (import.meta.env.DEV) {
        return "http://localhost:8081";
    }
    return "";
}

export const CONFIG = { api_prefix: get_api_prefix() };
