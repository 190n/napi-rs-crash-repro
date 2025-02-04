import { get } from './index.js';
console.log(await get(Promise.resolve(5)));
console.log(await get(Promise.resolve(5n)));
