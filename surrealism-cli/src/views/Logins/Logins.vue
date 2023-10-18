<template>
  <div :id="buildView(component)">
    <img src="../../assets/imgs/logo_surrealdb.png" alt="" style="width: 90%;">
    <div>
      功能暂时未开放
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'Logins'
}
</script>

<script lang="ts" setup>
import { ref, reactive } from 'vue'
import { build, buildWrap, buildView } from '../../styles/name'
let component = 'Logins'
</script>

<style lang="scss" scoped>
@import './Logins.scss';
</style>