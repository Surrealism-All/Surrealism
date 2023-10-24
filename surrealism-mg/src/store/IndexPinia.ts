import { defineStore } from "pinia";

export const indexStore = defineStore("index", {
  // other options...
  state: () => {
    return {
      menuList1: [
        {
          id: "1",
          defaultIcon: "home.svg",
          activeIcon: "home_active.svg",
          name: "Home",
        },
        {
          id: "2",
          defaultIcon: "sql_query.svg",
          activeIcon: "sql_query_active.svg",
          name: "SQL query",
        },
        {
          id: "3",
          defaultIcon: "tables.svg",
          activeIcon: "tables_active.svg",
          name: "Tables",
        },
        {
          id: "4",
          defaultIcon: "logins.svg",
          activeIcon: "logins_active.svg",
          name: "Logins",
        },
        {
          id: "5",
          defaultIcon: "tokens.svg",
          activeIcon: "tokens_active.svg",
          name: "Tokens",
        },

        {
          id: "6",
          defaultIcon: "scopes.svg",
          activeIcon: "scopes_active.svg",
          name: "Scopes",
        },
      ],
      menuList2: [
        {
          id: "7",
          defaultIcon: "logs.svg",
          activeIcon: "logs_active.svg",
          name: "Logs",
        },
        {
          id: "8",
          defaultIcon: "stats.svg",
          activeIcon: "stats_active.svg",
          name: "Stats",
        },
        {
          id: "9",
          defaultIcon: "setting.svg",
          activeIcon: "setting_active.svg",
          name: "Settings",
        },
      ],
      activeMenu: {
        id: "1",
        defaultIcon: "home.svg",
        activeIcon: "home_active.svg",
        name: "Home",
      },
    };
  },
  actions: {},
});
