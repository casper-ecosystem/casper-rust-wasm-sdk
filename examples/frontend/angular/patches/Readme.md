# Nx local patch

**Reason:** Workaround for an Nx import incompatible with the `@angular-devkit/architect` package exports.

**Affected file:** `nx/dist/src/adapter/ngcli-adapter.js`

**Change:** Replace:

```js
@angular-devkit/architect/node/index.js
```

with:

```js
@angular-devkit/architect/node
```

**When to remove:** Once an Nx release includes this correction upstream.

**How to verify:**

```bash
rg -n "architect/node/index" node_modules/nx/dist/src/adapter/ngcli-adapter.js
```

If the invalid import is gone in the upgraded Nx version, remove `patches/patch-package+nx.patch` and the `patch-package` postinstall setup (if no other patches depend on it).
