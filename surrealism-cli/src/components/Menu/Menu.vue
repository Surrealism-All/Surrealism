<template>
  <div :id="buildView(component)">
    <el-menu active-text-color="#fff" background-color="#161920" class="el-menu-vertical-demo" default-active="1" text-color="#8B8EA2" @open="handleOpen" @close="handleClose">
      <el-menu-item :index="item.id" v-for="item in store.menuList1" :key="item.id" @click="activeMenu(item.id,item)">
        <img class="common_img" v-if="activeNum==item.id" :src="getImageUrl(item.activeIcon)" alt="" srcset="">
        <img class="common_img" v-else :src="getImageUrl(item.defaultIcon)" alt="" srcset="">
        <span>{{ item.name }}</span>
      </el-menu-item>
      <div style="height:36px;"></div>
      <el-menu-item :index="item.id" v-for="item in store.menuList2" :key="item.id" @click="activeMenu(item.id,item)">
        <img class="common_img" v-if="activeNum==item.id" :src="getImageUrl(item.activeIcon)" alt="" srcset="">
        <img class="common_img" v-else :src="getImageUrl(item.defaultIcon)" alt="" srcset="">
        <span>{{ item.name }}</span>
      </el-menu-item>
    </el-menu>

  </div>
</template>

<script lang="ts">
export default {
  name: 'Menu'
}
</script>

<script lang="ts" setup>
import { useRouter } from 'vue-router'
import { ref, reactive } from 'vue'
import { buildView, build, buildWrap } from '../../styles/name'
import { indexStore } from '../../store/IndexPinia'

let router = useRouter()
let activeNum = ref('1')
let store = indexStore()
const getImageUrl = (image: string) => new URL(`../../assets/imgs/menu/${image}`, import.meta.url).href

const activeMenu = (id: string, item: any) => {
  activeNum.value = id
  store.activeMenu = item
  let targetPath = item.name.toLocaleLowerCase()
  router.push({ path: '/' + targetPath })
}

const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}

let component = 'Menu'
</script>

<style lang="scss" scoped>
@import './Menu.scss';
</style>