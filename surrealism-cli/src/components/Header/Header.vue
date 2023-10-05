<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component,'left')">
      <img :class="build('left','logo')" src="../../assets/imgs/logo_284.svg" alt="SurrealDB" srcset="">
    </div>
    <div :class="buildWrap(component,'right')">
      <div :class="build('right','left')">
        <el-button type="primary">
          <img class="common_img" src="../../assets/imgs/header/info.svg" alt="">
        </el-button>
      </div>
      <div :class="build('right','right')">
        <el-button type="primary">
          <img class="common_img2" src="../../assets/imgs/header/data.svg" alt="">
          <span class="common_font_color">Data</span>
        </el-button>
        <el-button type="primary">
          <img class="common_img2" src="../../assets/imgs/header/fields.svg" alt="">
          <span class="common_font_color">Fields</span>
        </el-button>
        <el-button type="primary">
          <img class="common_img2" src="../../assets/imgs/header/events.svg" alt="">
          <span class="common_font_color">Events</span>
        </el-button>
        <el-button type="primary">
          <img class="common_img2" src="../../assets/imgs/header/indexs.svg" alt="">
          <span class="common_font_color">Indexes</span>
        </el-button>
        <el-button type="primary">
          <img class="common_img2" src="../../assets/imgs/header/views.svg" alt="">
          <span class="common_font_color">Views</span>
        </el-button>
        <el-button type="primary">
          <img class="common_img" src="../../assets/imgs/header/search.svg" alt="">
        </el-button>
      </div>
    </div>
  </div>
</template>

<script  lang="ts">
export default {
  name: 'Header'
}
</script>

<script lang="ts" setup>
import { ref, reactive } from 'vue'
import { buildView, build, buildWrap } from '../../styles/name'
let component = 'Header'
</script>

<style lang="scss" scoped>
@import './Header.scss';
</style>