const lib = require('./index');

test('baseDirs', () => {
    expect(lib.baseDirs()).toMatchSnapshot();
});

test('projectDirs path', () => {
    expect(lib.projectDirs('project_name')).toMatchSnapshot();
});

test('projectDirs tripple', () => {
    expect(lib.projectDirs('com.example', 'Example inc.', 'example app')).toMatchSnapshot();
});

test('userDirs', () => {
    expect(lib.userDirs()).toMatchSnapshot();
});