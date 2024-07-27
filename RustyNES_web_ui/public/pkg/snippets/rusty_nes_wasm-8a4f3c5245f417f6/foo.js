export function add(a, b) {
    console.log("add called");
    return a + b;
}

export function throw_js_error() {
    throw new Error("This is a JS error");
}