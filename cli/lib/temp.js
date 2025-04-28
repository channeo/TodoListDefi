const path = require('path');
const fs = require('fs');
const DIR = path.join(__dirname, '../../temp');

module.exports = {
	location: () => DIR,
	load: (filename) => {
		try {
			filename = path.join(DIR, filename + '.json');
			const data = JSON.parse(fs.readFileSync(filename, 'utf8'));
			return data;
		} catch (er) {
			return null;
		}
	},
	save: (filename, data) => {
		try {
			fs.mkdirSync(DIR);
		} catch (er) {
			// Nothing
		}
		filename = path.join(DIR, filename + '.json');
		data = JSON.stringify(data, null, 2);
		return fs.writeFileSync(filename, data, 'utf8');
	},
};