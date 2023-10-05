<template>
  <div :id="buildView(component)">
    <el-menu active-text-color="#fff" background-color="#161920" class="el-menu-vertical-demo" default-active="1" text-color="#8B8EA2" @open="handleOpen" @close="handleClose">
      <el-menu-item :index="item.id" v-for="item in menuList1" :key="item.id" @click="activeMenu(item.id)">
        <img class="common_img" v-if="activeNum==item.id" :src="getImageUrl(item.activeIcon)" alt="" srcset="">
        <img class="common_img" v-else :src="getImageUrl(item.defaultIcon)" alt="" srcset="">
        <span>{{ item.name }}</span>
      </el-menu-item>
      <div style="height:36px;"></div>
      <el-menu-item :index="item.id" v-for="item in menuList2" :key="item.id" @click="activeMenu(item.id)">
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
import { ref, reactive } from 'vue'
import { buildView, build, buildWrap } from '../../styles/name'

let activeNum = ref('1')
let menuList1 = reactive([
  {
    id: '1',
    defaultIcon: 'home.svg',
    activeIcon: 'home_active.svg',
    name: 'Home'
  },
  {
    id: '2',
    defaultIcon: 'sql_query.svg',
    activeIcon: 'sql_query_active.svg',
    name: 'SQL query'
  },
  {
    id: '3',
    defaultIcon: 'tables.svg',
    activeIcon: 'tables_active.svg',
    name: 'Tables'
  },
  {
    id: '4',
    defaultIcon: 'logins.svg',
    activeIcon: 'logins_active.svg',
    name: 'Logins'
  },
  {
    id: '5',
    defaultIcon: 'tokens.svg',
    activeIcon: 'tokens_active.svg',
    name: 'Tokens'
  },

  {
    id: '6',
    defaultIcon: 'scopes.svg',
    activeIcon: 'scopes_active.svg',
    name: 'Scopes'
  }
])
let menuList2 = reactive([
  {
    id: '7',
    defaultIcon: 'logs.svg',
    activeIcon: 'logs_active.svg',
    name: 'Logs'
  },
  {
    id: '8',
    defaultIcon: 'stats.svg',
    activeIcon: 'stats_active.svg',
    name: 'Stats'
  },
  {
    id: '9',
    defaultIcon: 'setting.svg',
    activeIcon: 'setting_active.svg',
    name: 'Settings'
  }
])

const getImageUrl = (image: string) => new URL(`../../assets/imgs/menu/${image}`, import.meta.url).href

const activeMenu = (id: string) => {
  activeNum.value = id
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