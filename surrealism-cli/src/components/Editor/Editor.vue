<template>
  <div :id="buildView(componentName)">
    <div :class="buildWrap(componentName, 'code')">
      <div :class="buildWrap(componentName, 'row')">
        <div :class="build('row', 'item')" v-for="item in rowNum" :key="item">
          {{ item }}
        </div>
      </div>
      <code :contenteditable="props.editable" ref="codeRef" @keyup.enter="nextRow" @keyup.delete="deleteRow" @blur="countTextLength" @keyup.tab="addTab">
        <slot>
          <img src="../../../common/routerboot-ui.svg" alt="">
          {{ props.code }}
        </slot>
      </code>
      <div :class="buildWrap(componentName, 'text')">
        <!-- <span size="small" :type="props.type">{{ textLength }}个字符</span> -->
      </div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'Editor'
}
</script>

<script lang="ts" setup>
import { ref, reactive, PropType, computed, onMounted, onUpdated, onBeforeUpdate, defineProps, defineExpose } from 'vue'
import { buildView, build, buildWrap } from '../../styles/name'
const componentName = 'Editor'
let codeFontSize = ref(16)
let codeRef = ref(null)
let textLength = ref(0)

let rowNum = ref(0)
const props = defineProps({
  code: {
    type: String,
    default: ''
  },
  editable: {
    type: Boolean,
    default: false
  }
})
/**
 * 计算行数
 */
const countRowNumber = (): void => {
  let codeDom: any = codeRef.value!
  //获取dom元素的样式表
  let domStyles = window.getComputedStyle(codeDom, null)
  let domHeight = parseFloat(domStyles.height.replace('px', ''))
  rowNum.value = Math.round(domHeight / (codeFontSize.value * 1.5))
}
/**
 * 计算文字长度
 */
const countTextLength = () => {
  let codeDom: any = codeRef.value!
  textLength.value = codeDom.innerText.length
}

/**
 * 切换下一行，再次计算行数
 */
const nextRow = () => {
  countRowNumber()
}
/**
 * 退格事件
 */
const deleteRow = () => {
  countTextLength()
  countRowNumber()
}
onMounted(() => {
  countRowNumber()
  countTextLength()
})

let innerCode = computed(() => {
  let { code } = props
  return code
})

const addTab = () => {}

defineExpose({
  innerCode
})
</script>

<style lang="scss" scoped>
@use '../../styles/name.scss' as *;
@use '../../styles/src/var.scss' as *;

$component: 'Editor';

@include buildView($component) {
  cursor: pointer;
  height: 100%;
  width: 100%;
  border: none;
  margin: 0;
  padding: 0;
  font-size: 16px;
  transition: all 0.4s ease-in-out;
  background-color: $primary-color;
  display: inline-block;
  text-align: left;
  @include buildWrap($component, 'code') {
    position: relative;
    height: 100%;
    width: 100%;
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    overflow-y: scroll;
    scroll-behavior: smooth;
    scrollbar-width: thin;
    @include buildWrap($component, 'text') {
      position: absolute;
      bottom: 0;
      right: 0;
      margin: $margin-default-h $margin-default-w;
    }
  }
  @include buildWrap($component, 'code') {
    background-color: transparent;
    @include buildWrap($component, 'row') {
      min-width: 30px;
      width: auto;
      min-height: calc(100% - 2 * $padding-default-h);
      padding: $padding-default-h calc($padding-default-w / 2);
      background-color: $bg-color-dark;
      @include build('row', 'item') {
        height: auto;
        line-height: 1.5em;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 16px;
        color: $icon-color;
      }
    }
    code {
      min-height: 100%;
      text-indent: 0px;
      color: $icon-color;
      cursor: text;
      font-size: 16px;
      line-height: 1.5em;
      font-family: Verdana, Geneva, Tahoma, sans-serif;
      outline: none;
      // height: calc(100% - 2 * $padding-default-h);
      width: calc(100% - 2 * $padding-default-w - 28px);
      margin: 0;
      padding: $padding-default-h $padding-default-w;
      background-color: $primary-color;
      counter-reset: line;
      overflow-x: scroll;
      scroll-behavior: smooth;
      scrollbar-width: thin;
    }
  }
}
</style>
