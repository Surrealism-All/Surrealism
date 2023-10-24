import { defineStore } from "pinia";

export const SettingsStore = defineStore("settings", {
  // other options...
  state: () => {
    return {
      settings: [
        {
          id: 0,
          key: "path",
          value: "E://SurrealDB",
        },
        {
          id: 1,
          key: "language",
          value: "English",
        },
      ],
    };
  },
  actions: {},
});
