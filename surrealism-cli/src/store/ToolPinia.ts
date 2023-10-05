import { defineStore } from "pinia";

export const ToolStore = defineStore("tool", {
  // other options...
  state: () => {
    return {
      tools: [
        {
          id: 0,
          icon: "refresh.svg",
          name: "Refresh",
        },
        {
          id: 1,
          icon: "filter.svg",
          name: "Filter",
        },
        {
          id: 2,
          icon: "sorted.svg",
          name: "Sorted",
        },
        {
          id: 3,
          icon: "new.svg",
          name: "New Record",
        },
      ],
    };
  },
  actions: {},
});
