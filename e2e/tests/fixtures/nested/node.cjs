const { execSync } = require('child_process');
const path = require('path');


execSync('pnpm -v', { cwd: path.join(__dirname, 'c') });