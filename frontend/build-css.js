#!/usr/bin/env node
const { execSync } = require('child_process');

try {
  execSync('npx tailwindcss -i style/input.css -o style/output.css --minify', {
    stdio: 'inherit',
    cwd: __dirname
  });
} catch (error) {
  process.exit(1);
}
