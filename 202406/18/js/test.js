let foo = {
    'abc': 235
};
console.log(foo);
console.log(foo.abc);
let isDelete = (delete foo.abc);
console.log(foo);
console.log(foo.abc);