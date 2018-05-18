# appone
neon example 
see native/src/lib.rs & lib/index.html

```
npm install
neon build
npm start
```

# how to start

neon new appone
cd appone
npm install
neon build

modify lib/index.js
add    lib/index.html

modify package.json like this

```
  "dependencies": {
    "electron": "^2.0.1",
    "electron-rebuild": "^1.7.3",
    "neon-cli": "^0.1.22"
  },
  "devDependencies": {
    "electron": "^2.0.1",
    "electron-build-env": "^0.2.0",
    "neon-cli": "^0.1.22"
  },
  "scripts": {
    "start": "electron .",
    "install": "neon build"
  }
```

