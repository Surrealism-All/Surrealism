/**
 * ============================================
 * @author:syf20020816@outlook.com
 * @since:20230223
 * @version:0.2.0
 * @type:ts
 * @description:vue-router设置页面路由地址
 * ============================================
 */

import Index from "../views/Index/Index.vue";
import Home from "../views/Home/Home.vue";
import Logs from "../views/Logs/Logs.vue";
import Settings from "../views/Settings/Settings.vue";
import { RouteRecordRaw } from "vue-router";
export const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    redirect: "/home",
  },
  {
    path: "/home",
    component: Home,
  },
  {
    path: "/tables",
    component: Index,
  },
  {
    path: "/logs",
    component: Logs,
  },
  {
    path: "/settings",
    component: Settings,
  },
];
