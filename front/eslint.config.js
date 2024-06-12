import globals from "globals";
import js from "@eslint/js";
import stylisticJs from "@stylistic/eslint-plugin-js";

export default [
    js.configs.recommended,
    {
        languageOptions: {
            globals: {
                ...globals.browser,
                ...globals.node,
            },
        },
        plugins: {
            "@stylistic/js": stylisticJs,
        },
        rules: {
            "no-implicit-coercion": "error",
            "no-var": "error",
            "prefer-const": "warn",
            curly: ["warn", "all"],
            eqeqeq: ["warn", "always"],
            radix: "warn",
            strict: ["warn", "global"],
            "@stylistic/js/operator-linebreak": ["error", "before"],
        },
    },
];
