<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'left')">
      <div :class="build('left', 'title')" class="common_list">
        {{ store.activeMenu.name }}
      </div>
      <div :class="buildWrap('left', 'list')">
        <div class="common_list list" v-for="item in db.tables" :key="item.id" @click="activeTable(item.id)">
          <img class="common_img" v-if="db.activeTableNumber == item.id" src="../../assets/imgs/menu/tables_active.svg" alt="" />
          <img class="common_img" v-else src="../../assets/imgs/menu/tables.svg" alt="" />
          <span :style="avtiveTableFontStyle(item.id)">
            {{ item.name }}
          </span>
        </div>
      </div>
    </div>
    <div :class="buildWrap(component, 'right')">
      <div :class="buildWrap('right', 'tools')">
        <div :class="build('tools', 'item')" v-for="item in tool.tools" :key="item.id">
          <img class="common_img" :src="getImageUrl(item.icon)" alt="" />
          <span>{{ item.name }}</span>
        </div>
      </div>
      <div :class="buildWrap('right','main')">
        <el-table :data="tableData" style="width: 100%" max-height="250">
          <el-table-column fixed prop="date" label="Date" width="150" />
          <el-table-column prop="name" label="Name" width="120" />
          <el-table-column prop="state" label="State" width="120" />
          <el-table-column prop="city" label="City" width="120" />
          <el-table-column prop="address" label="Address" width="600" />
          <el-table-column prop="zip" label="Zip" width="120" />
          <el-table-column fixed="right" label="Operations" width="120">
            <template #default="scope">
              <el-button type="primary" size="small" @click.prevent="deleteRow(scope.$index)">
                Remove
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
      <div :class="buildWrap('right', 'page_tools')">
        <el-pagination v-model:current-page="currentPage" v-model:page-size="pageSize" :page-sizes="[100, 200, 300, 400]" :small="false" :disabled="false" :background="true" layout="total, sizes, prev, pager, next, jumper" :total="400" @size-change="handleSizeChange" @current-change="handleCurrentChange" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'Index'
}
</script>

<script lang="ts" setup>
import { ref, reactive, computed } from 'vue'
import { build, buildWrap, buildView } from '../../styles/name'
import { indexStore } from '../../store/IndexPinia'
import { DataBaseStore } from '../../store/DatabasePinia'
import { ToolStore } from '../../store/ToolPinia'

let component = 'Index'
let store = indexStore()
let db = DataBaseStore()
let tool = ToolStore()
const currentPage = ref(4)
const pageSize = ref(100)
const getImageUrl = (image: string) => new URL(`../../assets/imgs/tools/${image}`, import.meta.url).href

const activeTable = (index: number) => {
  db.activeTableNumber = index
}

const avtiveTableFontStyle = (index: number): string => {
  return index == db.activeTableNumber ? 'color:#fff;' : ''
}

const handleSizeChange = (val: number) => {
  console.log(`${val} items per page`)
}
const handleCurrentChange = (val: number) => {
  console.log(`current page: ${val}`)
}

const tableData = ref([
  {
    date: '2016-05-01',
    name: 'Tom',
    state: 'California',
    city: 'Los Angeles',
    address: 'No. 189, Grove St, Los Angeles',
    zip: 'CA 90036'
  },
  {
    date: '2016-05-02',
    name: 'Tom',
    state: 'California',
    city: 'Los Angeles',
    address: 'No. 189, Grove St, Los Angeles',
    zip: 'CA 90036'
  },
  {
    date: '2016-05-03',
    name: 'Tom',
    state: 'California',
    city: 'Los Angeles',
    address: 'No. 189, Grove St, Los Angeles',
    zip: 'CA 90036'
  }
])

const deleteRow = (index: number) => {
  tableData.value.splice(index, 1)
}
</script>

<style lang="scss" >
@import './Index.scss';
</style>
