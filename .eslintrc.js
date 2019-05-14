module.exports = {
  parser: '@typescript-eslint/parser',
  plugins: ['@typescript-eslint'],
  extends: ['airbnb-base', 'plugin:@typescript-eslint/recommended'],
  rules: {
    'no-param-reassign': 1,
    'comma-dangle': ['error', {
      arrays: 'always-multiline',
      objects: 'always-multiline',
      imports: 'always-multiline',
      exports: 'always-multiline',
      functions: 'never'
    }],
    'import/no-unresolved': 'off',
    'import/prefer-default-export': 'warn',
    'no-return-await': 'off',
    'operator-linebreak': ['error', 'after'],
    'object-curly-newline': ['error', {
      multiline: true,
      minProperties: 5,
      consistent: true
    }],
    '@typescript-eslint/indent': ['error', 2],
    '@typescript-eslint/explicit-function-return-type': ['error', {
      allowExpressions: true
    }]
  },
};
  