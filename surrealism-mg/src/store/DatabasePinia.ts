import { defineStore } from "pinia";

export const DataBaseStore = defineStore("database", {
  // other options...
  state: () => {
    return {
      tables: [
        {
          id: 0,
          name: "access",
        },
        {
          id: 1,
          name: "account",
        },
        {
          id: 2,
          name: "address",
        },
        {
          id: 3,
          name: "content",
        },
        {
          id: 4,
          name: "document",
        },
      ],
      activeTableNumber: 0,
    };
  },
  actions: {},
});
