<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component,'logo')">
      <img class="logo" src="../../assets/imgs/logo_surrealdb.png" alt="">
      <div class="title">
        The ultimate multi-model database
      </div>
    </div>
    <div :class="buildWrap(component,'tools')">
      <div :class="buildWrap(component,'item')">
        <img class="icon" src="../../assets/imgs/home/connect.svg" alt="">
        <div class="title">Connection</div>
      </div>
      <div :class="buildWrap(component,'item')">
        <img class="icon" src="../../assets/imgs/home/upload.svg" alt="">
        <div class="title">Upload</div>
      </div>
    </div>
    <div :class="buildWrap(component,'bottom')">
      <div class="version">Version:0.0.1_preview</div>
      <div class="maker">syf20020816@outlook.com</div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'Home'
}
</script>

<script lang="ts" setup>
import { ref, reactive } from 'vue'
import { build, buildWrap, buildView } from '../../styles/name'
let component = 'Home'
</script>

<style lang="scss" scoped>
@import './Home.scss';
</style>