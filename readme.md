##  uniapp环境下测试wasm使用

```
pnpm wasm    // 编译wasm
pnpm dev:h5  // 开发运行
```

如需要运行在mp-weixin环境，需要将js文件中的`WebAssembly`替换为`WXWebAssembly`, fetch行（`input = fetch(input);`）注释掉。