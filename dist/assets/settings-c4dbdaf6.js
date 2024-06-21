import{H as k,I as b,j as e,v as A,r as l,K as r,M as S,S as p,T as m,N as D,O as L,a0 as w,Q as P,X as T,L as R}from"./styles-db619cf7.js";import{C as B,A as M,T as W,c as s,L as c,a as o,b as a}from"./Toolbar-0927e661.js";import{C as f}from"./Container-7c1387da.js";import{C as d}from"./Checkbox-f79d812e.js";import"./useFormControl-f02c8f95.js";var j={},E=b;Object.defineProperty(j,"__esModule",{value:!0});var C=j.default=void 0,q=E(k()),H=e;C=j.default=(0,q.default)((0,H.jsx)("path",{d:"M19 6.41 17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"}),"Close");var v={},$=b;Object.defineProperty(v,"__esModule",{value:!0});var _=v.default=void 0,z=$(k()),F=e;_=v.default=(0,z.default)((0,F.jsx)("path",{d:"M12 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2m7-7H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2m-1.75 9c0 .23-.02.46-.05.68l1.48 1.16c.13.11.17.3.08.45l-1.4 2.42c-.09.15-.27.21-.43.15l-1.74-.7c-.36.28-.76.51-1.18.69l-.26 1.85c-.03.17-.18.3-.35.3h-2.8c-.17 0-.32-.13-.35-.29l-.26-1.85c-.43-.18-.82-.41-1.18-.69l-1.74.7c-.16.06-.34 0-.43-.15l-1.4-2.42c-.09-.15-.05-.34.08-.45l1.48-1.16c-.03-.23-.05-.46-.05-.69 0-.23.02-.46.05-.68l-1.48-1.16c-.13-.11-.17-.3-.08-.45l1.4-2.42c.09-.15.27-.21.43-.15l1.74.7c.36-.28.76-.51 1.18-.69l.26-1.85c.03-.17.18-.3.35-.3h2.8c.17 0 .32.13.35.29l.26 1.85c.43.18.82.41 1.18.69l1.74-.7c.16-.06.34 0 .43.15l1.4 2.42c.09.15.05.34-.08.45l-1.48 1.16c.03.23.05.46.05.69"}),"SettingsApplications");function O(){const i=A(),[h,x]=l.useState("Active"),[g,u]=l.useState("Active"),[y,I]=l.useState();return l.useEffect(()=>{r("get_settings").then(t=>{x(t.frontendWdgLvl),u(t.deadlockWdgLvl),I(t.configPath)}).catch(console.error)},[]),e.jsxs(S,{id:"content",component:"div",display:"flex",children:[e.jsx(B,{}),e.jsx(M,{position:"absolute",sx:{backgroundColor:i.palette.background.paper,height:"60px"},children:e.jsx(f,{component:"div",children:e.jsxs(W,{disableGutters:!0,sx:{justifyContent:"space-between"},children:[e.jsxs(p,{component:"div",direction:"row",children:[e.jsx(m,{variant:"h5",color:i.palette.text.secondary,children:"CANzero"}),e.jsx(m,{marginLeft:"0.5em",variant:"h5",color:i.palette.text.disabled,children:"Settings"})]}),e.jsx(p,{component:"div",direction:"row",spacing:2,children:e.jsx(D,{onClick:()=>r("close_settings").catch(console.error),children:e.jsx(C,{})})})]})})}),e.jsx(f,{component:"main",sx:{backgroundColor:i.palette.background.main,flexGrow:1,minHeight:"calc(100vh - 60px)",maxHeight:"calc(100vh - 60px)",width:"100%",overflow:"auto",position:"relative",marginTop:"60px"},children:e.jsxs(L,{children:[e.jsx(s,{disablePadding:!0,children:e.jsxs(c,{onClick:()=>{r("select_network_configuration").catch(console.error)},children:[e.jsx(o,{sx:{justifyContent:"center"},children:e.jsx(_,{})}),e.jsx(a,{primary:"Select network configuration",secondary:y})]})}),e.jsx(s,{disablePadding:!0,children:e.jsxs(c,{children:[e.jsx(o,{sx:{justifyContent:"center"},children:e.jsx(d,{checked:h!=="Active",onChange:t=>{r("set_frontend_lvl",{lvl:t.target.checked?"Ignore":"Active"}).then(n=>x(n)).catch(console.error)}})}),e.jsx(a,{primary:"Ignore Frontend Watchdog",secondary:"Timeout still signaled as a warining"})]})}),e.jsx(s,{disablePadding:!0,children:e.jsxs(c,{children:[e.jsx(o,{sx:{justifyContent:"center"},children:e.jsx(d,{disabled:h==="Active",checked:h==="Disable",onChange:t=>{r("set_frontend_lvl",{lvl:t.target.checked?"Disable":"Ignore"}).then(n=>x(n)).catch(console.error)}})}),e.jsx(a,{primary:"Disable Frontend Watchdog",secondary:"Danger: A frozen frontend might not lead to a shutdown"})]})}),e.jsx(s,{disablePadding:!0,children:e.jsxs(c,{children:[e.jsx(o,{sx:{justifyContent:"center"},children:e.jsx(d,{checked:g!=="Active",onChange:t=>{r("set_deadlock_lvl",{lvl:t.target.checked?"Ignore":"Active"}).then(n=>u(n)).catch(console.error)}})}),e.jsx(a,{primary:"Ignore Deadlock Watchdog",secondary:"Timeout still signaled as a warining"})]})}),e.jsx(s,{disablePadding:!0,children:e.jsxs(c,{children:[e.jsx(o,{sx:{justifyContent:"center"},children:e.jsx(d,{disabled:g==="Active",checked:g==="Disable",onChange:t=>{r("set_deadlock_lvl",{lvl:t.target.checked?"Disable":"Ignore"}).then(n=>u(n)).catch(console.error)}})}),e.jsx(a,{primary:"Disable Deadlock Watchdog",secondary:"Danger: A deadlocked backend might not lead to a shutdown"})]})})]})})]})}function G(){return e.jsx(w,{theme:P,children:e.jsx(O,{})})}T.createRoot(document.getElementById("root")).render(e.jsx(R.StrictMode,{children:e.jsx(G,{})}));