const { execSync } = require('child_process');
const path = require('path');


let res = execSync('pnpm -v', { cwd: path.join(__dirname, 'c') });
console.log(res.toString());
