(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const r of document.querySelectorAll('link[rel="modulepreload"]'))o(r);new MutationObserver(r=>{for(const l of r)if(l.type==="childList")for(const c of l.addedNodes)c.tagName==="LINK"&&c.rel==="modulepreload"&&o(c)}).observe(document,{childList:!0,subtree:!0});function n(r){const l={};return r.integrity&&(l.integrity=r.integrity),r.referrerPolicy&&(l.referrerPolicy=r.referrerPolicy),r.crossOrigin==="use-credentials"?l.credentials="include":r.crossOrigin==="anonymous"?l.credentials="omit":l.credentials="same-origin",l}function o(r){if(r.ep)return;r.ep=!0;const l=n(r);fetch(r.href,l)}})();function me(e,t){return console.log("add called"),e+t}let i,R=null;function V(){return(R===null||R.byteLength===0)&&(R=new Uint8Array(i.memory.buffer)),R}function ee(e,t){return e=e>>>0,V().subarray(e/1,e/1+t)}const y=new Array(128).fill(void 0);y.push(void 0,null,!0,!1);function F(e){return y[e]}let D=y.length;function ge(e){e<132||(y[e]=D,D=e)}function we(e){const t=F(e);return ge(e),t}const te=typeof TextDecoder<"u"?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};typeof TextDecoder<"u"&&te.decode();function $(e,t){return e=e>>>0,te.decode(V().subarray(e,e+t))}let v=0;function C(e,t){const n=t(e.length*1,1)>>>0;return V().set(e,n/1),v=e.length,n}let S=null;function pe(){return(S===null||S.byteLength===0)&&(S=new Float32Array(i.memory.buffer)),S}function ye(e,t){const n=t(e.length*4,4)>>>0;return pe().set(e,n/4),v=e.length,n}function he(e){D===y.length&&y.push(y.length+1);const t=D;return D=y[t],y[t]=e,t}let M=null;function J(){return(M===null||M.byteLength===0)&&(M=new Int32Array(i.memory.buffer)),M}const K=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(e=>i.__wbg_nes_free(e>>>0));class P{static __wrap(t){t=t>>>0;const n=Object.create(P.prototype);return n.__wbg_ptr=t,K.register(n,n.__wbg_ptr,n),n}__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,K.unregister(this),t}free(){const t=this.__destroy_into_raw();i.__wbg_nes_free(t)}static new_nes(t){const n=C(t,i.__wbindgen_malloc),o=v,r=i.nes_new_nes(n,o);return P.__wrap(r)}new_from_save_bytes(t){const n=C(t,i.__wbindgen_malloc),o=v,r=i.nes_new_from_save_bytes(this.__wbg_ptr,n,o);return P.__wrap(r)}step(){i.nes_step(this.__wbg_ptr)}frame_buffer_pointer(){return i.nes_frame_buffer_pointer(this.__wbg_ptr)>>>0}update_button(t,n){i.nes_update_button(this.__wbg_ptr,t,n)}load_audio_buffer(t){var n=ye(t,i.__wbindgen_malloc),o=v;i.nes_load_audio_buffer(this.__wbg_ptr,n,o,he(t))}sample_rate(){return i.nes_sample_rate(this.__wbg_ptr)}change_rom(t){const n=C(t,i.__wbindgen_malloc),o=v;i.nes_change_rom(this.__wbg_ptr,n,o)}get_state(){try{const r=i.__wbindgen_add_to_stack_pointer(-16);i.nes_get_state(r,this.__wbg_ptr);var t=J()[r/4+0],n=J()[r/4+1],o=ee(t,n).slice();return i.__wbindgen_free(t,n*1,1),o}finally{i.__wbindgen_add_to_stack_pointer(16)}}set_state(t){const n=C(t,i.__wbindgen_malloc),o=v;i.nes_set_state(this.__wbg_ptr,n,o)}throw_rust_error(){i.nes_throw_rust_error(this.__wbg_ptr)}}async function be(e,t){if(typeof Response=="function"&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(e,t)}catch(o){if(e.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",o);else throw o}const n=await e.arrayBuffer();return await WebAssembly.instantiate(n,t)}else{const n=await WebAssembly.instantiate(e,t);return n instanceof WebAssembly.Instance?{instance:n,module:e}:n}}function ve(){const e={};return e.wbg={},e.wbg.__wbindgen_copy_to_typed_array=function(t,n,o){new Uint8Array(F(o).buffer,F(o).byteOffset,F(o).byteLength).set(ee(t,n))},e.wbg.__wbg_onError_e4eef107580251b0=function(t,n){window.onError($(t,n))},e.wbg.__wbg_add_1f5a7f294ef132d5=function(t,n){return me(t>>>0,n>>>0)},e.wbg.__wbindgen_object_drop_ref=function(t){we(t)},e.wbg.__wbindgen_throw=function(t,n){throw new Error($(t,n))},e}function Ee(e,t){return i=e.exports,ne.__wbindgen_wasm_module=t,S=null,M=null,R=null,i}async function ne(e){if(i!==void 0)return i;typeof e>"u"&&(e=new URL("/RustyNES/pkg/rusty_nes_wasm_bg.wasm",import.meta.url));const t=ve();(typeof e=="string"||typeof Request=="function"&&e instanceof Request||typeof URL=="function"&&e instanceof URL)&&(e=fetch(e));const{instance:n,module:o}=await be(await e,t);return Ee(n,o)}var Le=typeof globalThis<"u"?globalThis:typeof window<"u"?window:typeof global<"u"?global:typeof self<"u"?self:{};function xe(e){return e&&e.__esModule&&Object.prototype.hasOwnProperty.call(e,"default")?e.default:e}var oe={exports:{}};(function(e,t){(function(n,o){e.exports=o()})(Le,function(){var n=function(){function o(_){return c.appendChild(_.dom),_}function r(_){for(var g=0;g<c.children.length;g++)c.children[g].style.display=g===_?"block":"none";l=_}var l=0,c=document.createElement("div");c.style.cssText="position:fixed;top:0;left:0;cursor:pointer;opacity:0.9;z-index:10000",c.addEventListener("click",function(_){_.preventDefault(),r(++l%c.children.length)},!1);var d=(performance||Date).now(),f=d,s=0,a=o(new n.Panel("FPS","#0ff","#002")),m=o(new n.Panel("MS","#0f0","#020"));if(self.performance&&self.performance.memory)var h=o(new n.Panel("MB","#f08","#201"));return r(0),{REVISION:16,dom:c,addPanel:o,showPanel:r,begin:function(){d=(performance||Date).now()},end:function(){s++;var _=(performance||Date).now();if(m.update(_-d,200),_>f+1e3&&(a.update(1e3*s/(_-f),100),f=_,s=0,h)){var g=performance.memory;h.update(g.usedJSHeapSize/1048576,g.jsHeapSizeLimit/1048576)}return _},update:function(){d=this.end()},domElement:c,setMode:r}};return n.Panel=function(o,r,l){var c=1/0,d=0,f=Math.round,s=f(window.devicePixelRatio||1),a=80*s,m=48*s,h=3*s,_=2*s,g=3*s,b=15*s,E=74*s,L=30*s,x=document.createElement("canvas");x.width=a,x.height=m,x.style.cssText="width:80px;height:48px";var u=x.getContext("2d");return u.font="bold "+9*s+"px Helvetica,Arial,sans-serif",u.textBaseline="top",u.fillStyle=l,u.fillRect(0,0,a,m),u.fillStyle=r,u.fillText(o,h,_),u.fillRect(g,b,E,L),u.fillStyle=l,u.globalAlpha=.9,u.fillRect(g,b,E,L),{dom:x,update:function(O,_e){c=Math.min(c,O),d=Math.max(d,O),u.fillStyle=l,u.globalAlpha=1,u.fillRect(0,0,a,b),u.fillStyle=r,u.fillText(f(O)+" "+o+" ("+f(c)+"-"+f(d)+")",h,_),u.drawImage(x,g+s,b,E-s,L,g,b,E-s,L),u.fillRect(g+E-s,b,s,L),u.fillStyle=l,u.globalAlpha=.9,u.fillRect(g+E-s,b,s,f((1-O/_e)*L))}}},n})})(oe);var Ae=oe.exports;const Q=xe(Ae),X={l:0,k:1,Shift:2,Enter:3,w:4,s:5,a:6,d:7},A=256,W=240,Ie=A*W*4;async function re(e){const t=await fetch(e);if(!t.ok)throw new Error(`Failed to fetch ROM: ${t.statusText}`);const n=await t.arrayBuffer();return new Uint8Array(n)}let U=null,w=null,N=null,T=!1,k=!1,se,ae,ie,le,ce,p,Y,Z=!1,I,B;const de=()=>{if(console.log("startVideo"),!Z){Z=!0;let e=document.querySelector(".stats");I=new Q,I.showPanel(0),I.dom.style.cssText="",e.appendChild(I.dom),B=new Q,B.showPanel(1),B.dom.style.cssText="",e.appendChild(B.dom)}k||(k=!0,Re())},Be=()=>{k&&(k=!1,U&&(cancelAnimationFrame(U),U=null))};window.onError=e=>{G(),alert(e),console.error(e)};const Re=()=>{let e=window.performance.now(),t=0,n=0,o=e,r=0,c=Math.floor(1e3/60),d=performance.now(),f=0;const s=a=>{if(I.begin(),B.begin(),U=requestAnimationFrame(s),f=a-d,f<c){console.log("Rendering too fast");return}if(d=a-f%c,t++,se(),r=a-o,n+=r,o=a,t%60===0){let m=1e3/(n/60);console.log(m.toFixed(2)),t=0,n=0}I.end(),B.end()};s(d)},Se=()=>{w=new AudioContext({sampleRate:48e3}),w.onstatechange=()=>{console.log(w==null?void 0:w.state)},N=w.createScriptProcessor(1024,0,1),N.onaudioprocess=e=>{ae(e)},N.connect(w.destination),console.log("audio setup")},q=async()=>{w||Se(),!T&&(await(w==null?void 0:w.resume()),T=!0,console.log("audio playing"))},ue=async()=>{w&&T&&(await w.suspend(),T=!1,console.log("audio paused"))},fe=async()=>{de(),!H&&await q()},G=async()=>{await ue(),Be()};let Me=document.getElementById("play"),De=document.getElementById("pause"),Pe=document.getElementById("mute"),H=!1;const Te=()=>{window.addEventListener("keydown",le),window.addEventListener("keyup",ie),Me.addEventListener("click",fe),De.addEventListener("click",G),Pe.addEventListener("click",async()=>{k&&(w?T?await ue():await q():await q(),H=!H)}),[...document.getElementsByClassName("rom")].forEach(a=>{a.addEventListener("click",async m=>{let h=a.getAttribute("data-name");ke(`roms/${h}.nes`)})});let t=document.getElementById("up"),n=document.getElementById("down"),o=document.getElementById("left"),r=document.getElementById("right"),l=document.getElementById("a"),c=document.getElementById("b"),d=document.getElementById("select"),f=document.getElementById("start");const s=(a,m)=>{a.addEventListener("touchstart",()=>{a.classList.toggle("pressed"),navigator.vibrate(70),p.update_button(m,!0)}),a.addEventListener("touchend",()=>{a.classList.toggle("pressed"),p.update_button(m,!1)})};s(l,0),s(c,1),s(d,2),s(f,3),s(t,4),s(n,5),s(o,6),s(r,7)},ke=async e=>{await G();const t=await re(e);ce(t),await fe()},Oe=async e=>{console.log("init called with url ",e);const n=document.querySelector("#screen").getContext("2d"),o=n.createImageData(A,W);let r=document.getElementById("canvas-container");console.log(r);const l=Math.min(window.innerWidth/A,window.innerHeight/W,3);let c=(A-10)*l;r.style.width=c+"px",window.addEventListener("resize",()=>{const a=Math.min(window.innerWidth/A,window.innerHeight/W,3);let m=(A-10)*a;r.style.width=m+"px"});const d=await re(e),f=await ne();window.wasm=f,Y=f.memory,p=P.new_nes(d),window.nes=p;const s=(a,m)=>{a.key in X&&(a.preventDefault(),p.update_button(X[a.key],m))};le=a=>{s(a,!0)},ie=a=>{s(a,!1)},se=()=>{p.step(),o.data.set(new Uint8ClampedArray(Y.buffer,p.frame_buffer_pointer(),Ie)),n.putImageData(o,0,0)},ae=a=>{p.load_audio_buffer(a.outputBuffer.getChannelData(0))},ce=a=>{p.change_rom(a)},Te()},Ce=async()=>{console.log("inside main");let e=document.getElementById("save"),t=document.getElementById("load");e.onclick=Fe,t.onclick=We;const n=document.getElementById("screen");n.getContext("2d"),document.getElementById("downloadBtn").addEventListener("click",()=>{console.log("Download button clicked");const o=document.createElement("canvas");o.style.imageRendering="pixelated";const r=2;o.width=n.width*r,o.height=n.height*r;const l=o.getContext("2d");l.imageSmoothingEnabled=!1,l.drawImage(n,0,0,n.width,n.height,0,0,o.width,o.height);const c=o.toDataURL("image/png"),d=document.createElement("a");d.href=c,d.download="canvas-image.png",d.click()}),console.log("before init"),await Oe("roms/mario.nes"),console.log("after init"),de()};let z;const Fe=()=>{z=p.get_state(),console.log(z)},We=()=>{try{p.set_state(z)}catch(e){console.log(e)}};document.addEventListener("DOMContentLoaded",()=>{Ce().then(()=>console.log("after main"))});let j=indexedDB.open("wasm",1);j.onerror=e=>{console.log("error",e),console.log(j.error)};j.onsuccess=e=>{console.log("success",e),console.log(j.result)};
