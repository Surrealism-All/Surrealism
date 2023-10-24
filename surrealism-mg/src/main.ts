import { createApp } from "vue";
import App from "./App.vue";
//import vue router
import router from "./router/index";
//import global style
import "./styles/global.scss";
//import Element-plus
import ElementPlus from "element-plus";
//导入Element-plus默认样式，若使用自定义则注释掉
// import 'element-plus/dist/index.css'
//使用pinia
import pinia from "./store/pinia";
//导入图标库（这里并非使用ElementPlus提供的图标库而是自己的）
// import "./assets/defaultIcons/iconfont.css";

const app = createApp(App);
app.use(router);
app.use(ElementPlus);
app.use(pinia);
app.mount("#app");
