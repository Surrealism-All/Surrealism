<template>
  <div id="app">
    <div id="header">
      <Header></Header>
    </div>
    <div id="main_view">
      <div id="menu"></div>
      <router-view></router-view>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive } from 'vue'
import Header from './components/Header/Header.vue'
</script>

<style lang="scss" scoped>
@use './styles/src/var.scss' as *;
#app {
  height: 100vh;
  width: 100vw;
  background: $bg-color-dark;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  flex-wrap: wrap;
  #header {
    height: 66px;
    width: 100%;
  }
  #main_view {
    height: calc(100% - 66px);

    width: 100%;
  }
}
</style>