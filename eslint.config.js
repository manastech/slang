// eslint.config.js
import js from '@eslint/js';
import tseslint from '@typescript-eslint/eslint-plugin';
import tsparser from '@typescript-eslint/parser';
import * as pluginJest from 'eslint-plugin-jest';

export default [
  js.configs.recommended,
  {
    files: ['**/*.ts', '**/*.tsx', '**/*.mts'],
    languageOptions: {
      parser: tsparser,
      parserOptions: {
        project: [
          './tsconfig.json',
          './crates/codegen/runtime/npm/package/tsconfig.json',
          './crates/solidity/outputs/npm/package/tsconfig.json',
          './crates/solidity/outputs/npm/tests/tsconfig.json',
          './documentation/public/assets/javascripts/tsconfig.json',
          './documentation/public/user-guide/tsconfig.json'
        ],
        ecmaVersion: 2020,
        sourceType: 'module',
      },
    },
    plugins: {
      '@typescript-eslint': tseslint,
      jest: pluginJest
    },
    rules: {
      ...tseslint.configs.recommended.rules,
      // Add or override rules here
    },
  },
  {
    ignores: [
      'node_modules/',
      '**/target/',
      '.hermit/',
      '**/*.js',
      // Not sure about these.
      'jest.config.ts',
      '**/*.d.ts'
    ],
  },
];
