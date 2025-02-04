import { get } from './index.js';
try {
	console.log(await get(Promise.resolve(5)));
	console.log(await get(Promise.resolve(5n)));
} catch (e) {
	console.error('caught error from await:', e);
}
