
                (function() {
                    var wasm;
                    const __exports = {};
                    

class DoublePendulumLagrangian {

                static __construct(ptr) {
                    return new DoublePendulumLagrangian(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }
            get g() {
    return wasm.__wbg_get_doublependulumlagrangian_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_doublependulumlagrangian_g(this.ptr, arg0);
}get m1() {
    return wasm.__wbg_get_doublependulumlagrangian_m1(this.ptr);
}
set m1(arg0) {
    return wasm.__wbg_set_doublependulumlagrangian_m1(this.ptr, arg0);
}get m2() {
    return wasm.__wbg_get_doublependulumlagrangian_m2(this.ptr);
}
set m2(arg0) {
    return wasm.__wbg_set_doublependulumlagrangian_m2(this.ptr, arg0);
}get t1() {
    return wasm.__wbg_get_doublependulumlagrangian_t1(this.ptr);
}
set t1(arg0) {
    return wasm.__wbg_set_doublependulumlagrangian_t1(this.ptr, arg0);
}get t2() {
    return wasm.__wbg_get_doublependulumlagrangian_t2(this.ptr);
}
set t2(arg0) {
    return wasm.__wbg_set_doublependulumlagrangian_t2(this.ptr, arg0);
}get dt1() {
    return wasm.__wbg_get_doublependulumlagrangian_dt1(this.ptr);
}
set dt1(arg0) {
    return wasm.__wbg_set_doublependulumlagrangian_dt1(this.ptr, arg0);
}get dt2() {
    return wasm.__wbg_get_doublependulumlagrangian_dt2(this.ptr);
}
set dt2(arg0) {
    return wasm.__wbg_set_doublependulumlagrangian_dt2(this.ptr, arg0);
}get l1() {
    return wasm.__wbg_get_doublependulumlagrangian_l1(this.ptr);
}
set l1(arg0) {
    return wasm.__wbg_set_doublependulumlagrangian_l1(this.ptr, arg0);
}get l2() {
    return wasm.__wbg_get_doublependulumlagrangian_l2(this.ptr);
}
set l2(arg0) {
    return wasm.__wbg_set_doublependulumlagrangian_l2(this.ptr, arg0);
}
            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_doublependulumlagrangian_free(ptr);
            }
        static new(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    return DoublePendulumLagrangian.__construct(wasm.doublependulumlagrangian_new(arg0, arg1, arg2, arg3, arg4, arg5, arg6));
}
time_step(arg0) {
    return wasm.doublependulumlagrangian_time_step(this.ptr, arg0);
}
}
__exports.DoublePendulumLagrangian = DoublePendulumLagrangian;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer)
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

__exports.__wbindgen_cos = function(x) { return Math.cos(x); };

__exports.__wbindgen_sin = function(x) { return Math.sin(x); };

                    function init(wasm_path) {
                        return fetch(wasm_path)
                            .then(response => response.arrayBuffer())
                            .then(buffer => WebAssembly.instantiate(buffer, { './rustc_h_alr91rm5hgd': __exports }))
                            .then(({instance}) => {
                                wasm = init.wasm = instance.exports;
                                return;
                            });
                    };
                    self.wasm_bindgen = Object.assign(init, __exports);
                })();
            