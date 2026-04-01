import { defineConfig } from "eslint/config";
import globals from "globals";
import eslint from "typescript-eslint";
import stylistic from "@stylistic/eslint-plugin";
import typescript from "@typescript-eslint/eslint-plugin";
import parserTs from "@typescript-eslint/parser";
import mustUse from "@mconst/eslint-plugin-must-use";

export default defineConfig([
  {
    ignores: [
      "**/*.md",
      "**/*.json",
      "**/*.yaml",
      "**/*.css",
      "**/Dockerfile/**",
      "**/tsconfig.tsbuildinfo",
      "./types/**",
      "./dist/**",
      "./fonts/**",
    ],
  },
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
      parser: parserTs,
      ecmaVersion: 2024,
      sourceType: "commonjs",
      parserOptions: {
        project: "./tsconfig.json",
      },
    },
    plugins: {
      "": eslint,
      "@typescript-eslint": typescript,
      "@stylistic": stylistic,
      "@mustUse": mustUse,
    },
    extends: [
      "@typescript-eslint/strict-type-checked",
      "@stylistic/disable-legacy",
      "@stylistic/recommended",
      "/recommended",
    ],
    rules: {
      "@stylistic/quote-props": ["error", "as-needed"],
      "@stylistic/function-call-spacing": ["error", "never"],
      "@stylistic/function-call-argument-newline": ["error", "consistent"],
      "@stylistic/function-paren-newline": ["error", "consistent"],
      "@stylistic/operator-linebreak": ["error", "before", {
        overrides: {
          "=": "after",
        },
      }],

      "@stylistic/lines-around-comment": ["error", {
        beforeBlockComment: true,
        beforeLineComment: true,
        allowBlockStart: true,
        allowClassStart: true,
        allowObjectStart: true,
        allowArrayStart: true,
        allowEnumStart: true,
        allowInterfaceStart: true,
        allowModuleStart: true,
        allowTypeStart: true,
      }],

      "@stylistic/padding-line-between-statements": [
        "error",
        {
          blankLine: "always",
          prev: "*",
          next: "return",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "break",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "class",
        },
        {
          blankLine: "always",
          prev: "directive",
          next: "*",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "iife",
        },
        {
          blankLine: "always",
          prev: "iife",
          next: "*",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "multiline-block-like",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "multiline-const",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "multiline-expression",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "multiline-let",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "throw",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "interface",
        },
        {
          blankLine: "always",
          prev: "*",
          next: "type",
        },
        {
          blankLine: "never",
          prev: "function-overload",
          next: "function-overload",
        },
      ],

      "@stylistic/semi": ["error", "always"],
      "@stylistic/quotes": ["error", "double"],

      "@stylistic/max-len": ["error", {
        code: 106,
        tabWidth: 2,
        ignoreStrings: true,
        ignoreTemplateLiterals: true,
        ignoreRegExpLiterals: true,
        ignoreComments: true,
      }],

      "@stylistic/multiline-ternary": ["error", "always-multiline"],
      "@stylistic/array-bracket-newline": ["error", "consistent"],
      "@stylistic/array-element-newline": ["error", "consistent"],

      "@stylistic/object-curly-newline": ["error", {
        ObjectExpression: {
          consistent: true,
          multiline: true,
          minProperties: 2,
        },

        ObjectPattern: {
          consistent: true,
          multiline: true,
          minProperties: 2,
        },

        ImportDeclaration: "never",
        ExportDeclaration: "always",
      }],

      "@stylistic/object-property-newline": ["error", {
        allowAllPropertiesOnSameLine: true,
      }],

      "@typescript-eslint/adjacent-overload-signatures": ["error"],

      "@typescript-eslint/array-type": ["error", {
        default: "array-simple",
      }],

      "@typescript-eslint/ban-ts-comment": ["error", {
        "ts-expect-error": "allow-with-description",
        "ts-ignore": true,
        "ts-nocheck": true,
        "ts-check": false,
        minimumDescriptionLength: 3,
      }],

      "@typescript-eslint/ban-tslint-comment": ["error"],

      "@typescript-eslint/no-restricted-types": ["error", {
        types: {
          String: {
            message: "Use string instead",
            fixWith: "string",
          },

          Boolean: {
            message: "Use boolean instead",
            fixWith: "boolean",
          },

          Number: {
            message: "Use number instead",
            fixWith: "number",
          },

          Symbol: {
            message: "Use symbol instead",
            fixWith: "symbol",
          },

          BigInt: {
            message: "Use bigint instead",
            fixWith: "bigint",
          },

          Function: {
            message: "The `Function` type accepts any function-like value.\nIt provides no type safety when calling the function, which can be a common source of bugs.\nIt also accepts things like class declarations, which will throw at runtime as they will not be called with `new`.\nIf you are expecting the function to accept certain arguments, you should explicitly define the function shape.",
          },

          Object: {
            message: "The `Object` type actually means \"any non-nullish value\", so it is marginally better than `unknown`.\n- If you want a type meaning \"any object\", you probably want `Record<string, unknown>` instead.\n- If you want a type meaning \"any value\", you probably want `unknown` instead.",
          },

          "{}": {
            message: "`{}` actually means \"any non-nullish value\".\n- If you want a type meaning \"any object\", you probably want `Record<string, unknown>` instead.\n- If you want a type meaning \"any value\", you probably want `unknown` instead.",
          },
        },
      }],

      "@typescript-eslint/consistent-generic-constructors": ["error", "constructor"],
      "@typescript-eslint/consistent-indexed-object-style": ["error", "record"],

      "@typescript-eslint/consistent-type-assertions": ["error", {
        assertionStyle: "as",
        objectLiteralTypeAssertions: "never",
      }],

      "@typescript-eslint/consistent-type-definitions": ["error", "type"],

      "@typescript-eslint/consistent-type-exports": ["error", {
        fixMixedExportsWithInlineTypeSpecifier: true,
      }],

      "@typescript-eslint/consistent-type-imports": ["error", {
        prefer: "type-imports",
        disallowTypeAnnotations: true,
        fixStyle: "inline-type-imports",
      }],

      "@typescript-eslint/explicit-function-return-type": ["error", {
        allowExpressions: true,
        allowHigherOrderFunctions: true,
        allowTypedFunctionExpressions: true,
        allowDirectConstAssertionInArrowFunctions: true,
      }],

      "@typescript-eslint/explicit-member-accessibility": ["error"],
      "@typescript-eslint/explicit-module-boundary-types": ["error"],
      "@typescript-eslint/method-signature-style": ["error", "property"],

      "@typescript-eslint/naming-convention": ["error", {
        selector: "variableLike",
        leadingUnderscore: "allow",
        trailingUnderscore: "allow",
        format: ["camelCase", "PascalCase", "UPPER_CASE"],
      }],

      "@typescript-eslint/no-confusing-void-expression": ["error", {
        ignoreArrowShorthand: false,
        ignoreVoidOperator: false,
      }],

      "@typescript-eslint/no-dynamic-delete": ["error"],

      "@typescript-eslint/no-empty-interface": ["error", {
        allowSingleExtends: true,
      }],

      "@typescript-eslint/no-extra-non-null-assertion": ["error"],

      "@typescript-eslint/no-extraneous-class": ["error", {
        allowWithDecorator: true,
      }],

      "@typescript-eslint/no-import-type-side-effects": ["error"],
      "@typescript-eslint/no-redeclare": ["off"],

      "@typescript-eslint/no-this-alias": ["error", {
        allowDestructuring: true,
      }],

      "@typescript-eslint/no-unnecessary-qualifier": ["error"],
      "@typescript-eslint/no-unsafe-call": ["error"],

      "@typescript-eslint/no-unused-expressions": ["error", {
        allowShortCircuit: true,
        allowTernary: true,
        allowTaggedTemplates: true,
        enforceForJSX: false,
      }],

      "@typescript-eslint/no-unused-vars": ["error", {
        args: "none",
        caughtErrors: "none",
        ignoreRestSiblings: true,
        vars: "all",
      }],

      "@typescript-eslint/no-use-before-define": ["error", {
        functions: false,
        classes: false,
        enums: false,
        variables: false,
        typedefs: false,
      }],

      "@typescript-eslint/non-nullable-type-assertion-style": ["error"],

      "@typescript-eslint/only-throw-error": ["error", {
        allowThrowingAny: false,
        allowThrowingUnknown: false,
      }],

      "@typescript-eslint/prefer-enum-initializers": ["error"],
      "@typescript-eslint/prefer-find": ["error"],
      "@typescript-eslint/prefer-for-of": ["error"],
      "@typescript-eslint/prefer-function-type": ["error"],
      "@typescript-eslint/prefer-includes": ["error"],
      "@typescript-eslint/prefer-namespace-keyword": ["error"],

      "@typescript-eslint/prefer-nullish-coalescing": ["error", {
        ignoreConditionalTests: false,
        ignoreMixedLogicalExpressions: false,
      }],

      "@typescript-eslint/prefer-optional-chain": ["error"],
      "@typescript-eslint/prefer-readonly": ["error"],
      "@typescript-eslint/prefer-reduce-type-parameter": ["error"],
      "@typescript-eslint/prefer-string-starts-ends-with": ["error"],
      "@typescript-eslint/promise-function-async": ["error"],

      "@typescript-eslint/require-array-sort-compare": ["error", {
        ignoreStringArrays: true,
      }],

      "@typescript-eslint/restrict-plus-operands": ["error", {
        skipCompoundAssignments: false,
      }],

      "@typescript-eslint/restrict-template-expressions": ["error", {
        allowNumber: true,
      }],

      "@typescript-eslint/return-await": ["error", "always"],

      "@typescript-eslint/strict-boolean-expressions": ["error", {
        allowString: false,
        allowNumber: false,
        allowNullableObject: false,
        allowNullableBoolean: false,
        allowNullableString: false,
        allowNullableNumber: false,
        allowAny: false,
      }],

      "@typescript-eslint/switch-exhaustiveness-check": "error",

      "@typescript-eslint/triple-slash-reference": ["error", {
        lib: "never",
        path: "never",
        types: "never",
      }],

      "@typescript-eslint/unbound-method": ["error", {
        ignoreStatic: false,
      }],

      "array-callback-return": ["error", {
        allowImplicit: false,
        allowVoid: false,
        checkForEach: false,
      }],

      "constructor-super": ["error"],
      curly: ["error", "multi-line"],
      "default-case-last": ["error"],

      eqeqeq: ["error", "always", {
        null: "ignore",
      }],

      "new-cap": ["error", {
        newIsCap: true,
        capIsNew: false,
        properties: true,
      }],

      "no-async-promise-executor": ["error"],
      "no-caller": ["error"],
      "no-case-declarations": ["error"],
      "no-class-assign": ["error"],
      "no-compare-neg-zero": ["error"],
      "no-cond-assign": ["error"],
      "no-const-assign": ["error"],

      "no-constant-condition": ["error", {
        checkLoops: false,
      }],

      "no-control-regex": ["error"],
      "no-debugger": ["error"],
      "no-delete-var": ["error"],
      "no-dupe-args": ["error"],
      "no-dupe-keys": ["error"],
      "no-duplicate-case": ["error"],
      "no-useless-backreference": ["error"],

      "no-empty": ["error", {
        allowEmptyCatch: true,
      }],

      "no-empty-character-class": ["error"],
      "no-empty-pattern": ["error"],
      "no-eval": ["error"],
      "no-ex-assign": ["error"],
      "no-extend-native": ["error"],
      "no-extra-bind": ["error"],
      "no-extra-boolean-cast": ["error"],
      "no-fallthrough": ["error"],
      "no-func-assign": ["error"],
      "no-global-assign": ["error"],
      "no-import-assign": ["error"],
      "no-invalid-regexp": ["error"],
      "no-irregular-whitespace": ["error"],
      "no-iterator": ["error"],

      "no-labels": ["error", {
        allowLoop: false,
        allowSwitch: false,
      }],

      "no-lone-blocks": ["error"],
      "no-misleading-character-class": ["error"],
      "no-prototype-builtins": ["error"],
      "no-useless-catch": ["error"],
      "no-multi-str": ["error"],
      "no-new": ["error"],
      "no-new-func": ["error"],
      "no-new-symbol": ["error"],
      "no-new-wrappers": ["error"],
      "no-obj-calls": ["error"],
      "no-object-constructor": ["error"],
      "no-octal": ["error"],
      "no-octal-escape": ["error"],
      "no-proto": ["error"],
      "no-regex-spaces": ["error"],
      "no-return-assign": ["error", "except-parens"],

      "no-self-assign": ["error", {
        props: true,
      }],

      "no-self-compare": ["error"],
      "no-sequences": ["error"],
      "no-shadow-restricted-names": ["error"],
      "no-sparse-arrays": ["error"],
      "no-template-curly-in-string": ["error"],
      "no-this-before-super": ["error"],
      "no-throw-literal": ["off"],
      "no-undef-init": ["error"],
      "no-unexpected-multiline": ["error"],
      "no-unmodified-loop-condition": ["error"],

      "no-unneeded-ternary": ["error", {
        defaultAssignment: false,
      }],

      "no-unreachable": ["error"],
      "no-unreachable-loop": ["error"],
      "no-unsafe-finally": ["error"],
      "no-unsafe-negation": ["error"],
      "no-useless-call": ["error"],
      "no-useless-computed-key": ["error"],
      "no-useless-escape": ["error"],
      "no-useless-rename": ["error"],
      "no-useless-return": ["error"],
      "no-var": ["warn"],

      "no-void": ["error", {
        allowAsStatement: true,
      }],

      "no-with": ["error"],
      "object-shorthand": ["warn", "properties"],

      "one-var": ["error", {
        initialized: "never",
      }],

      "prefer-const": ["error", {
        destructuring: "all",
        ignoreReadBeforeAssign: false,
      }],

      "prefer-regex-literals": ["error", {
        disallowRedundantWrapping: true,
      }],

      "symbol-description": ["error"],
      "unicode-bom": ["error", "never"],

      "use-isnan": ["error", {
        enforceForSwitchCase: true,
        enforceForIndexOf: true,
      }],

      "valid-typeof": ["error", {
        requireStringLiterals: true,
      }],

      yoda: ["error", "never"],
    },
  },
  {
    files: ["**/*.mjs"],

    rules: {
      "@typescript-eslint/explicit-module-boundary-types": "off",
      "@typescript-eslint/no-unsafe-return": "off",
      "@typescript-eslint/no-unsafe-call": "off",
      "@typescript-eslint/no-unsafe-member-access": "off",
    },
  },
  {
    files: ["**/*.d.ts"],
    rules: {
      "@typescript-eslint/no-unused-vars": ["off"],
    },
  },
]);

