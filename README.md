# use-client

[中文](https://github.com/coder-xiaotian/swc-useclient/blob/main/README-ZH.md)

![NPM](https://img.shields.io/npm/l/use-client)
[![NPM](https://img.shields.io/npm/l/use-client)](https://www.npmjs.com/package/use-client)
[![Cirrus CI - Task and Script Build Status](https://img.shields.io/cirrus/github/coder-xiaotian/swc-useclient)](https://github.com/coder-xiaotian/swc-useclient/actions/workflows/rust.yml)

A swc plugin that automatically converts React component libraries into "React Client Component". For example, you can automatically convert components from @mui into "React Client Component" without having to wrap a component that uses "use client".

## Installation

npm:

```
npm install -D use-client
```

yarn:

```
yarn add -D use-client
```

pnpm:

```
pnpm i -D use-client
```

## Configuration

- include: ["@mui/material"]

## Examples

next.js configuration:

```js
const nextConfig = {
  experimental: {
    swcPlugins: [
      [
        "use-client",
        {
          include: ["@mui/material"],
        },
      ],
    ],
  },
};
```

swc configuration:

```js
{
  "$schema": "https://json.schemastore.org/swcrc",
  "jsc": {
    "experimental": {
      "plugins": [
        ["use-client", {
          "include": ["@mui/material"]
        }]
      ]
    }
  }
}
```
