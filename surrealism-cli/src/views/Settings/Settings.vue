<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component,'left')">
      <div :class="build('left', 'title')" class="common_list">
        <span>Settings</span>
        <el-button type="primary">Save</el-button>
      </div>
      <div :class="build('left','main')">
        <div :class="buildWrap('left', 'list')">
          <div class="common_list list" v-for="kitem in store.settings" :key="kitem.id">
            <span>
              {{ kitem.key }}
            </span>
          </div>
        </div>
        <div :class="buildWrap('right', 'list')">
          <div class="common_list list" v-for="vitem in store.settings" :key="vitem.id">
            <div contenteditable="true" class="edit_setting">
              {{ vitem.value }}
            </div>
          </div>
        </div>
      </div>
    </div>
    <div :class="buildWrap(component,'right')">
      <Editor :code="editContent" :editable="true"></Editor>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'Settings'
}
</script>

<script lang="ts" setup>
import { ref, reactive } from 'vue'
import { build, buildWrap, buildView } from '../../styles/name'
import { SettingsStore } from '../../store/SettingsPinia'
import Editor from '../../components/Editor/Editor.vue'
let component = 'Settings'

let editContent = ref('change settings')
let store = SettingsStore()
</script>

<style lang="scss" scoped>
@import './Settings.scss';
</style>